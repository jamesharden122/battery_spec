pub mod batteries;
pub mod ev_chargers;
pub mod photovoltaic;
pub mod general_fun {

    use anyhow;
    use chrono::{DateTime, Utc};
    use polars::frame::DataFrame;
    use polars::prelude::*;

    #[derive(Clone, Debug)]
    pub struct EnergyConsumer {
        pub demand: PowerComponent,
        pub time_cycles: f32,
        pub total_input: f32,
        pub total_output: f32,
    }

    #[derive(Clone, Debug)]
    pub struct PowerComponent {
        pub input_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
        pub output_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
    }

    impl PowerComponent {
        fn new() -> Self {
            Self {
                input_power_w_ts: None,
                output_power_w_ts: None,
            }
        }

        pub fn new_ts(
            input_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
            output_power_w_ts: Option<Vec<(DateTime<Utc>, f32)>>,
        ) -> Self {
            Self {
                input_power_w_ts,
                output_power_w_ts,
            }
        }

        pub fn power(volts: f32, amps: f32, power_factor: f32) -> Result<f32, anyhow::Error> {
            let const_val: f32 = 3.0;
            Ok(const_val.sqrt() * volts * amps * power_factor)
        }

        pub fn merge_power_components(
            components: Vec<PowerComponent>,
            time_cycle: f32,
        ) -> EnergyConsumer {
            let mut colnames = components[0]
                .clone()
                .input_power_w_ts
                .unwrap()
                .iter()
                .map(|(dt, _val)| *dt)
                .collect::<Vec<DateTime<Utc>>>();
            let mut input_power_ts: Vec<Series> = Vec::new();
            let mut output_power_ts: Vec<Series> = Vec::new();
            let mut i = 1;
            for component in components.iter() {
                i += 1;
                input_power_ts.push(Series::new(
                    format!("charger{}", i).as_str().into(),
                    component
                        .clone()
                        .input_power_w_ts
                        .unwrap()
                        .iter()
                        .map(|(_dt, val)| *val)
                        .collect::<Vec<f32>>(),
                ));
                output_power_ts.push(Series::new(
                    format!("charger{}", i).as_str().into(),
                    component
                        .clone()
                        .output_power_w_ts
                        .unwrap()
                        .iter()
                        .map(|(_dt, val)| *val)
                        .collect::<Vec<f32>>(),
                ));
            }
            let df_input: PolarsResult<DataFrame> = DataFrame::new(input_power_ts);
            let _df_output: PolarsResult<DataFrame> = DataFrame::new(output_power_ts);
            let total_input = 0.5;
            let total_ouput = 0.5;
            let df_input = df_input.unwrap().clone().transpose(
                None,
                Some(either::Either::Right(
                    colnames
                        .iter_mut()
                        .map(|nm| nm.to_string())
                        .collect::<Vec<String>>(),
                )),
            );
            let mut sums: Vec<(DateTime<Utc>, f32)> = Vec::new();
            // Iterate over the DataFrame's columns to compute the sum of each
            for (column, date) in df_input.unwrap().get_columns().iter().zip(colnames.iter()) {
                let sum = column.sum::<f32>().unwrap();
                sums.push((*date, sum));
            }
            EnergyConsumer {
                demand: PowerComponent {
                    input_power_w_ts: Some(sums),
                    output_power_w_ts: None,
                },
                time_cycles: time_cycle,
                total_input,
                total_output: total_ouput,
            }
        }
    }
}
