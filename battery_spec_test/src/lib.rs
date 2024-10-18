use crate::energy_components::batteries::BatteryStorage;
use crate::energy_components::ev_chargers::Charger;
use crate::energy_components::general_fun::PowerComponent;
use crate::energy_components::photovoltaic::pv_base_system::PvSystem;
use crate::time_processes::*;
use chrono::{DateTime, Local, Utc};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;
pub mod energy_components;
pub mod surreal_data_structs;
pub mod time_processes;
use csv::Writer;
use plotters::prelude::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use surreal_data_structs::*;

// Define your data structure
pub type StatData = (usize, f64, f64, f32, f32, f64);
pub type Data = (DateTime<Utc>, f32, f32, f32, bool);
pub type Lambdas = Vec<f64>;
pub fn create_stat_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    // Write the header
    wtr.write_record([
        "Chargers Count",
        "Energy System Size",
        "Battery Size",
        "Duration Energy % Needed by Grid",
        "max_output",
        "average usage",
    ])?;

    // Flush the writer to ensure the header is written
    wtr.flush()?;
    Ok(())
}

pub fn append_to_stat_csv(data: Vec<StatData>, file_path: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(file_path).exists() {
        let file = OpenOptions::new().append(true).open(file_path)?;
        let mut wtr = Writer::from_writer(file);
        // Write the data
        for (
            chargers_count,
            energy_system_size,
            battery_size,
            duration_energy_needed,
            max_output,
            average_usage,
        ) in data
        {
            wtr.write_record(&[
                chargers_count.to_string(),
                energy_system_size.to_string(),
                battery_size.to_string(),
                duration_energy_needed.to_string(),
                max_output.to_string(),
                average_usage.to_string(),
            ])?;
        }

        // Flush the writer to ensure all data is written
        wtr.flush()?;
    } else {
        let _ = create_stat_csv(file_path);
        let file = OpenOptions::new().append(true).open(file_path)?;
        let mut wtr = Writer::from_writer(file);

        // Write the data
        for (
            chargers_count,
            energy_system_size,
            battery_size,
            duration_energy_needed,
            max_output,
            average_usage,
        ) in data
        {
            wtr.write_record(&[
                chargers_count.to_string(),
                energy_system_size.to_string(),
                battery_size.to_string(),
                duration_energy_needed.to_string(),
                max_output.to_string(),
                average_usage.to_string(),
            ])?;
        }
        // Flush the writer to ensure all data is written
        wtr.flush()?;
    }
    Ok(())
}

pub fn write_to_csv(data: Vec<Data>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    // Write the header
    wtr.write_record([
        "DateTime",
        "Storage",
        "Input Power",
        "Output Power",
        "Negative Net Storage",
    ])?;

    // Write the data
    for (datetime, storage, input_power, output_power, dummy) in data {
        wtr.write_record(&[
            datetime.to_rfc3339(),
            storage.to_string(),
            input_power.to_string(),
            output_power.to_string(),
            dummy.to_string(),
        ])?;
    }
    // Flush the writer to ensure all data is written
    wtr.flush()?;
    Ok(())
}

/// Plots a given vector of `f32` values to a PNG file.
///
/// # Arguments
///
/// * `values` - A vector of `f32` containing the values to plot.
/// * `file_path` - The output file path for the PNG image.
///
/// # Errors
///
/// This functiodatan returns an error if the plotting process fails.

pub fn plot_values_with_datetimes_to_png(
    data: Vec<(DateTime<Utc>, f32)>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let datetime: Vec<DateTime<Utc>> = data
        .clone()
        .into_iter()
        .map(|(x, _y)| x)
        .collect::<Vec<DateTime<Utc>>>();
    let values: Vec<f32> = data
        .clone()
        .into_iter()
        .map(|(_x, y)| y)
        .collect::<Vec<f32>>();
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    //
    let (to_date, from_date) = (
        datetime
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .with_timezone(&Local),
        datetime
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .with_timezone(&Local),
    );
    //
    root.fill(&WHITE)?;
    let min_value = *values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_value = *values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("EV Charging Simulation", ("sans-serif", 30.0))
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(120)
        .build_cartesian_2d(from_date..to_date, min_value..max_value)?;

    chart
        .configure_mesh()
        .y_desc("Mega Watt Hours")
        .axis_desc_style(("sans-serif", 20).into_font())
        .y_label_style(TextStyle::from(("sans-serif", 15).into_font()).color(&RED))
        .x_labels(20) // Set the number of labels to a suitable number for your data
        .x_label_style(("sans-serif", 10).into_font())
        .draw()?;

    chart.draw_series(LineSeries::new(
        data.into_iter().map(|(x, y)| (x.with_timezone(&Local), y)),
        &BLUE,
    ))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}

pub fn plot_values_to_png(
    values: Vec<f32>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let min_value = *values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_value = *values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Vec<f32> Day", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(120)
        .caption("EV Charging Simulation", ("sans-serif", 30.0))
        .build_cartesian_2d(0..values.len(), min_value..max_value)?;

    chart
        .configure_mesh()
        .y_desc("Mega Watt Hours")
        .axis_desc_style(("sans-serif", 20).into_font())
        .y_label_style(
            TextStyle::from(("sans-serif", 15).into_font()).color(&RED), // Customize color
        )
        .x_label_style(("sans-serif", 20).into_font())
        .draw()?;

    chart.draw_series(LineSeries::new(values.into_iter().enumerate(), &BLUE))?;

    root.present()?;
    Ok(())
}

pub async fn run_simulation(
    charging_station: &mut Vec<Charger>,
    base_photovoltaic: PvSystem,
    battery_storage: &mut BatteryStorage,
    lambda: Vec<f64>,
    _file_path: &str,
    date1: &str,
    date2: &str,
    datab: &Surreal<Client>,
    site: &str,
) -> Result<(), anyhow::Error> {
    //Simulate the process
    let df = modulated_markov_poison_process(date1, date2, lambda.clone()).await?;
    let time_matrix = df.0;
    let date_matrix = df.2;
    let mut i = 0;
    let _production_time = (0..time_matrix.ncols())
        .map(|col| time_matrix.column(col).sum())
        .collect::<Vec<f64>>();
    let mut charging_station = charging_station
        .iter_mut()
        .map(|charger| {
            let mut j = 0;
            charger.power.input_power_w_ts = Some(
                charger
                    .clone()
                    .power
                    .input_power_w_ts
                    .unwrap()
                    .iter_mut()
                    .map(|(dt, pow)| {
                        let power_output = (*pow) * (time_matrix[(i, j)] as f32);
                        let temp = date_matrix[(i, j)];
                        if i % 23 == 0 && i != 0 {
                            j += 1;
                            i = 0;
                        } else {
                            i += 1;
                        }
                        match true {
                            _ if &temp == dt => (), //println!("{:?}", "This matched"),
                            _ => println!("{:?}", "Doesn't match"),
                        }
                        (*dt, power_output)
                    })
                    .collect::<Vec<(DateTime<Utc>, f32)>>(),
            );
            charger
        })
        .collect::<Vec<&mut Charger>>();
    // ... further result processing and output
    // Optionally plot the results to a PNG file
    let power_component_vec: Vec<PowerComponent> = charging_station
        .iter_mut()
        .map(|charger| charger.power.clone())
        .collect();
    let charging_station_comp = PowerComponent::merge_power_components(power_component_vec, 1.0);
    let generator = base_photovoltaic.clone().into_power_component();
    // Update the BatteryStorage instance with the new power component data
    //println!("{:?}", generator.clone() );
    *battery_storage = battery_storage
        .clone()
        .update_power_component(generator, charging_station_comp.demand);
    sim_to_csv(&mut battery_storage.clone());
    _ = gen_stat(
        &mut battery_storage.clone(),
        base_photovoltaic,
        charging_station,
        lambda,
        "stat_df.csv",
    );
    _ = battery_storage_to_db(&mut battery_storage.clone(), datab, site).await?;
    //let _ = plot_values_with_datetimes_to_png(battery_storage.battery_state.storage.unwrap(),"storage.png");
    //let _ = plot_values_with_datetimes_to_png(battery_storage.battery_state.input_power_w_ts.unwrap(),"input.png");
    //let _ = plot_values_with_datetimes_to_png(battery_storage.battery_state.output_power_w_ts.unwrap(),"output.png");
    Ok(())
}

pub fn sim_to_csv(batt_system: &mut BatteryStorage) {
    let storage_iter = batt_system
        .clone()
        .battery_state
        .storage
        .unwrap()
        .into_iter();
    let input_iter = batt_system
        .clone()
        .battery_state
        .input_power_w_ts
        .unwrap()
        .into_iter();
    let output_iter = batt_system
        .clone()
        .battery_state
        .output_power_w_ts
        .unwrap()
        .into_iter();
    let neg_stat_iter = batt_system
        .clone()
        .battery_state
        .neg_stat_ts
        .unwrap()
        .into_iter();
    let data: Vec<_> = storage_iter
        .zip(input_iter.zip(output_iter.zip(neg_stat_iter)))
        .map(
            |((s_date, s_val), ((i_date, i_val), ((o_date, o_val), (_j_date, j_val))))| {
                assert_eq!(s_date, i_date);
                assert_eq!(s_date, o_date);
                (s_date, s_val, i_val, o_val, j_val)
            },
        )
        .collect();
    let _ = write_to_csv(data, "test_data.csv");
}

pub fn gen_stat(
    batt_system: &mut BatteryStorage,
    pv_system: PvSystem,
    ev_chargers: Vec<&mut Charger>,
    lamb_vec: Vec<f64>,
    file_path: &str,
) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
    fn sum_bools(bools: &Vec<bool>) -> usize {
        bools.iter().map(|&b| b as usize).sum()
    }
    fn max_f32_in_vec(vec: &Vec<f32>) -> Option<f32> {
        vec.iter().cloned().fold(None, |max, val| match max {
            None => Some(val),
            Some(max_val) => Some(max_val.max(val)),
        })
    }
    let storage_iter = batt_system
        .clone()
        .battery_state
        .storage
        .unwrap()
        .into_iter();
    let input_iter = batt_system
        .clone()
        .battery_state
        .input_power_w_ts
        .unwrap()
        .into_iter();
    let output_iter = batt_system
        .clone()
        .battery_state
        .output_power_w_ts
        .unwrap()
        .into_iter();
    let neg_stat_iter = batt_system
        .clone()
        .battery_state
        .neg_stat_ts
        .unwrap()
        .into_iter();
    let data = storage_iter.clone().zip(
        input_iter
            .clone()
            .zip(output_iter.clone().zip(neg_stat_iter.clone())),
    );
    let data_neg_stat = data
        .clone()
        .map(
            |((s_date, _s_val), ((i_date, _i_val), ((o_date, _o_val), (_j_date, j_val))))| {
                assert_eq!(s_date, i_date);
                assert_eq!(s_date, o_date);
                j_val
            },
        )
        .collect();
    let data_storage: Vec<_> = storage_iter
        .zip(input_iter.zip(output_iter.zip(neg_stat_iter)))
        .map(
            |((s_date, s_val), ((i_date, _i_val), ((o_date, _o_val), (_j_date, _j_val))))| {
                assert_eq!(s_date, i_date);
                assert_eq!(s_date, o_date);
                s_val
            },
        )
        .collect();
    //println!("{:?}", sum_bools(&data_neg_stat));
    //println!("{:?}", (sum_bools(&data_neg_stat) as f32)/(data.len() as f32));
    append_to_stat_csv(
        vec![(
            ev_chargers.len(),
            (pv_system.num_panels * pv_system.panel_watts).into(),
            (batt_system.capacity * batt_system.watt_hours).into(),
            (sum_bools(&data_neg_stat) as f32) / (data.len() as f32),
            max_f32_in_vec(&data_storage).expect("REASON"),
            lamb_vec.iter().sum::<f64>() / 60.0,
        )],
        file_path,
    )
}

pub async fn setup_and_run_simulation(
    db: &Surreal<Client>,
    template_charger_180kw: &mut Charger,
    solar_system: PvSystem,
    battery_storage: &mut BatteryStorage,
    start_date: &str,
    end_date: &str,
    battery_param_vec: Vec<(f32, f32)>,
    pv_param_vec: Vec<(usize, usize, f32)>,
    ev_charger_param_vec: Vec<usize>,
    lamb_vec: Vec<Lambdas>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Setup SurrealDB entry
    let _temp: Option<EvPvLdes> = db
        .create((
            "ev_pv_ldes",
            format!("{}_{}", "san-benito", uuid::Uuid::new_v4()),
        ))
        .content(EvPvLdes {
            date_time: surrealdb::sql::Datetime::from(Utc::now()),
            storage: 5.0,
            input_power: 10.0,
            output_power: 15.0,
            negative_net_storage: false,
        })
        .await
        .unwrap();
    println!("got here");

    let _ = template_charger_180kw.add_input_power_ts(start_date, end_date);
    let mut charging_station: Vec<Charger> = vec![template_charger_180kw.clone(); 20];
    let _ = create_stat_csv("specification_neg_stat.csv");

    run_simulation(
        &mut charging_station,
        solar_system,
        battery_storage,
        lamb_vec[0].to_vec(),
        "",
        start_date,
        end_date,
        db,
        "norwalk-arts-center",
    )
    .await?;

    let mut i = 1;
    for battery in battery_param_vec.iter() {
        for pv in pv_param_vec.iter() {
            for ev_charger in ev_charger_param_vec.iter() {
                for rates in lamb_vec.iter() {
                    let solar_system: PvSystem = PvSystem::new(
                        (pv.0 * pv.1) as f32,
                        pv.2,
                        0.64,
                        7.0,
                        7.0,
                        "full_irradiance_data.csv",
                    );
                    let mut battery_storage =
                        BatteryStorage::new(battery.0, battery.1, 80.0, 48.0, 90.0);
                    let mut charging_station: Vec<Charger> =
                        vec![template_charger_180kw.clone(); *ev_charger];
                    let _ = create_stat_csv("specification_neg_stat.csv");
                    run_simulation(
                        &mut charging_station,
                        solar_system,
                        &mut battery_storage,
                        rates.to_vec(),
                        "",
                        start_date,
                        end_date,
                        db,
                        "norwalk-art-complex",
                    )
                    .await?;
                    i += 1;
                    println!("{}", i);
                }
            }
        }
    }
    Ok(())
}
