use crate::energy_components::general_fun::PowerComponent;
use anyhow::Result;
use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)] // Ensure Charger can be cloned/copied
pub struct Charger {
    pub maximum_power_w: f32,
    pub output_voltage_min_vdc: f32,
    pub output_voltage_max_vdc: f32,
    pub max_output_current_a: f32,
    pub input_voltage_vac: f32,
    pub input_frequency_hz: u16,
    pub fla_a: f32,
    pub breaker_rating_a: u16,
    pub rated_power_kva: f32,
    pub power_factor_at_full_load: f32,
    pub efficiency_at_nominal_power: f32,
    pub power: PowerComponent,
}

impl Charger {
    pub fn new(
        maximum_power_w: f32,
        output_voltage_min_vdc: f32,
        output_voltage_max_vdc: f32,
        max_output_current_a: f32,
        input_voltage_vac: f32,
        input_frequency_hz: u16,
        fla_a: f32,
        breaker_rating_a: u16,
        rated_power_kva: f32,
        power_factor_at_full_load: f32,
        efficiency_at_nominal_power: f32,
    ) -> Self {
        Self {
            maximum_power_w,
            output_voltage_min_vdc,
            output_voltage_max_vdc,
            max_output_current_a,
            input_voltage_vac,
            input_frequency_hz,
            fla_a,
            breaker_rating_a,
            rated_power_kva,
            power_factor_at_full_load,
            efficiency_at_nominal_power,
            power: PowerComponent {
                input_power_w_ts: None,
                output_power_w_ts: None,
            },
        }
    }

    // Example method to add input power with timestamp
    pub fn add_input_power_ts(&mut self, date1: &str, date2: &str) -> Result<()> {
        let date_fmt = "%Y-%m-%d %H:%M:%S%:z"; // Ensure this format matches your input data
        let d1 = DateTime::parse_from_str(date1, date_fmt)?;
        let d2 = DateTime::parse_from_str(date2, date_fmt)?;
        let input_power_w = PowerComponent::power(
            self.input_voltage_vac,
            self.fla_a,
            self.efficiency_at_nominal_power,
        )
        .unwrap(); // Consider handling errors instead of unwrapping.
        let output_power_w = PowerComponent::power(
            self.output_voltage_max_vdc,
            self.max_output_current_a,
            self.power_factor_at_full_load,
        )
        .unwrap();
        //instantiate dates and energy inputs and outputs
        let mut curr_time = d1;
        let mut input_power_w_ts: Vec<(DateTime<Utc>, f32)> = Vec::new();
        let mut output_power_w_ts: Vec<(DateTime<Utc>, f32)> = Vec::new();

        loop {
            if curr_time == d2 {
                break;
            }
            input_power_w_ts.push((curr_time.into(), input_power_w));
            output_power_w_ts.push((curr_time.into(), output_power_w));
            curr_time += TimeDelta::try_hours(1).unwrap();
        }
        self.power.input_power_w_ts = Some(input_power_w_ts);
        self.power.output_power_w_ts = Some(output_power_w_ts);
        Ok(())
    }
}
