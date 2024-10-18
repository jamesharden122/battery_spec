pub mod pv_base_system {
    use crate::energy_components::general_fun::PowerComponent;
    use chrono::NaiveDateTime;
    use chrono::Utc;
    use chrono::{DateTime, FixedOffset};
    use polars::prelude::*;
    use serde::Deserialize;
    use serde::Deserializer;
    use std::path::PathBuf;

    #[derive(Clone, Debug, Deserialize)]
    struct Record {
        #[serde(rename = "date", deserialize_with = "deserialize_datetime")]
        index: DateTime<FixedOffset>,
        #[serde(rename = "kt")]
        kt: f64,
        #[serde(rename = "fd")]
        fd: f64,
        #[serde(rename = "g0")]
        G0: f64,
        #[serde(rename = "d0")]
        D0: f64,
        #[serde(rename = "b0")]
        B0: f64,
        #[serde(rename = "binc")]
        B_inc: f64,
        #[serde(rename = "dinc")]
        D_inc: f64,
        #[serde(rename = "rinc")]
        R_inc: f64,
        #[serde(rename = "g_inc")]
        G_inc: f64,
    }

    fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date_fmt = "%Y-%m-%d %H:%M:%S%:z"; // Ensure this format matches your input data
        DateTime::parse_from_str(&s, date_fmt).map_err(serde::de::Error::custom)
    }

    #[derive(Clone, Debug)]
    pub struct SolarParams {
        pub irradiance: polars::prelude::DataFrame, // in W/m^2
                                                    // Additional parameters as needed
    }

    #[derive(Clone, Debug)]
    pub struct PvSystem {
        pub num_panels: f32,
        pub panel_watts: f32,
        pub panel_cost: f32,
        pub transformer_efficiency: f32,
        pub panel_width: f32,
        pub panel_length: f32,
        pub params: SolarParams,
    }
    impl PvSystem {
        /// Calculates the solar generation based on irradiance and other factors.
        /// This method would encapsulate the logic for calculating solar power generation.        ///
        /// Creates a new instance of the system with given parameters.
        pub fn new(
            num_panels: f32,
            panel_watts: f32,
            transformer_efficiency: f32,
            panel_width: f32,
            panel_length: f32,
            params: &str,
        ) -> Self {
            let _delta_time = 1.0;
            let mut reader = PathBuf::new();
            let mut schema = Schema::with_capacity(9);
            reader.push(params);
            schema.with_column(
                "index".into(),
                DataType::Datetime(datatypes::TimeUnit::Milliseconds, None),
            );
            schema.with_column("kt".into(), DataType::Float32);
            schema.with_column("fd".into(), DataType::Float32);
            schema.with_column("G0".into(), DataType::Float32);
            schema.with_column("D0".into(), DataType::Float32);
            schema.with_column("B0".into(), DataType::Float32);
            schema.with_column("B_inc".into(), DataType::Float32);
            schema.with_column("D_inc".into(), DataType::Float32);
            schema.with_column("R_inc".into(), DataType::Float32);
            schema.with_column("G_inc".into(), DataType::Float32);

            // Read the CSV file into a DataFrame
            let mut irradiance_records = CsvReadOptions::default()
                .with_has_header(true)
                .with_schema(Some(schema.into()))
                .try_into_reader_with_file_path(Some(reader))
                .unwrap()
                .finish()
                .unwrap();
            let irradiance_records = irradiance_records.rename("index", "date".into()).unwrap();
            // Print the DaStaFrame to check it's loaded correctly
            //println!("{:?}", irradiance_records);
            Self {
                num_panels,
                panel_watts,
                panel_cost: 0.0,
                transformer_efficiency,
                panel_width,
                panel_length,
                params: SolarParams {
                    irradiance: irradiance_records.to_owned(),
                },
            }
        }

        pub fn into_power_component(self) -> PowerComponent {
            fn naive_to_utc(naive_datetime: NaiveDateTime) -> DateTime<Utc> {
                DateTime::from_naive_utc_and_offset(naive_datetime, Utc)
            }
            //println!("{:?}", self.params.irradiance.get_column_names());
            let date_column = self.params.irradiance.column("date").unwrap();
            //println!("{:?}", date_column);
            let mpp_column = self
                .params
                .irradiance
                .column("G_inc")
                .unwrap()
                .f32()
                .unwrap()
                .to_vec();
            let input_power_w_ts: Vec<(DateTime<Utc>, f32)> = Vec::new();
            let output_power_w_ts: Vec<(DateTime<Utc>, f32)> = mpp_column
                .iter()
                .zip(date_column.datetime().unwrap().as_datetime_iter())
                .map(|(x, y)| {
                    (
                        naive_to_utc(y.unwrap()),
                        x.unwrap() * self.num_panels * self.panel_watts,
                    )
                })
                .collect::<Vec<(DateTime<Utc>, f32)>>();
            PowerComponent {
                input_power_w_ts: Some(input_power_w_ts),
                output_power_w_ts: Some(output_power_w_ts),
            }
        }
    }
}
