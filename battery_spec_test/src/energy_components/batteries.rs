use crate::energy_components::ev_chargers::Charger;
use crate::energy_components::general_fun::EnergyConsumer;
use crate::Utc;
use chrono::DateTime;

use super::general_fun::PowerComponent;
#[derive(Clone, Debug)]
pub struct BatteryPowerComponent {
    pub storage: Option<Vec<(DateTime<Utc>, f32)>>,
    pub input_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
    pub output_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
    pub neg_stat_ts: Option<Vec<(DateTime<Utc>, bool)>>,
}
#[derive(Clone, Debug)]
pub struct BatteryStorage {
    pub capacity: f32,
    pub watt_hours: f32,
    pub depth_of_discharge: f32,
    pub battery_system_voltage: f32,
    pub efficiency: f32,
    pub battery_state: BatteryPowerComponent,
}

impl BatteryStorage {
    // General constructor
    pub fn new(
        capacity: f32,
        watt_hours: f32,
        depth_of_discharge: f32,
        battery_system_voltage: f32,
        efficiency: f32,
    ) -> BatteryStorage {
        BatteryStorage {
            capacity,
            watt_hours,
            depth_of_discharge,
            battery_system_voltage,
            efficiency,

            battery_state: BatteryPowerComponent {
                storage: None,
                input_power_w_ts: None,
                output_power_w_ts: None,
                neg_stat_ts: None,
            },
        }
    }

    pub fn update_power_component(
        mut self,
        generator: PowerComponent,
        utility: PowerComponent,
    ) -> BatteryStorage {
        let temp_vals = generator
            .output_power_w_ts
            .unwrap()
            .iter()
            .zip(utility.input_power_w_ts.unwrap().iter())
            .map(|(&(d1, g1), &(_d2, o1))| (d1, g1, o1))
            .collect::<Vec<(DateTime<Utc>, f32, f32)>>();

        let mut storage: Vec<(DateTime<Utc>, f32)> = Vec::new();
        let mut input: Vec<(DateTime<Utc>, f32)> = Vec::new();
        let mut output: Vec<(DateTime<Utc>, f32)> = Vec::new();
        let mut neg_dummy: Vec<(DateTime<Utc>, bool)> = Vec::new();
        storage.push((temp_vals[0].0, temp_vals[0].1 - temp_vals[0].2));
        input.push((temp_vals[0].0, temp_vals[0].1));
        output.push((temp_vals[0].0, temp_vals[0].2));
        neg_dummy.push((temp_vals[0].0, storage[0].1 < 0.0));
        for idx in 1..temp_vals.len() {
            let temp_storage = storage[idx - 1].1 + temp_vals[idx].1 - temp_vals[idx].2;
            if self.capacity * self.watt_hours <= temp_storage {
                storage.push((temp_vals[idx].0, self.capacity * self.watt_hours));
            } else {
                storage.push((temp_vals[idx].0, temp_storage));
            };
            input.push((temp_vals[idx].0, temp_vals[idx].1));
            output.push((temp_vals[idx].0, temp_vals[idx].2));
            let is_negative = storage[idx].1 < 0.0;
            neg_dummy.push((temp_vals[idx].0, is_negative));
        }

        self.battery_state.storage = Some(storage);
        self.battery_state.input_power_w_ts = Some(input);
        self.battery_state.output_power_w_ts = Some(output);
        self.battery_state.neg_stat_ts = Some(neg_dummy);
        self
    }
}

// Constructor that calculates capacity
