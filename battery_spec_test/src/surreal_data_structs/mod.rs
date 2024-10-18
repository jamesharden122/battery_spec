use crate::BatteryStorage;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Datetime;
use surrealdb::{Surreal};
use uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatData {
    pub chargers_count: usize,
    pub energy_system_size: f64,
    pub battery_size: f64,
    pub duration_energy_needed: f32,
    pub max_output: f32,
    pub average_usage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvPvLdes {
    pub date_time: Datetime,
    pub storage: f32,
    pub input_power: f32,
    pub output_power: f32,
    pub negative_net_storage: bool,
}

pub async fn setup_surrealdb(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
    // Create namespace and database
    let surr_sql = r#"
        DEFINE NAMESPACE microgrid;
        DEFINE DATABASE batteries;
        USE NAMESPACE microgrid;
        USE DATABASE batteries;
        "#;

    db.query(surr_sql).await?;

    Ok(())
}

pub async fn db_update_stats(
    db: &Surreal<Client>,
    data: Vec<EvPvLdes>,
    site: &str,
) -> Result<(), surrealdb::Error> {
    // Write stat_data to SurrealDB
    for record in data {
        let _temp: Option<StatData> = db
            .create((
                "sufficiency",
                format!("{}_{}", site, uuid::Uuid::new_v4().to_string()),
            ))
            .content(record)
            .await?;
    }
    Ok(())
}

pub async fn db_update_timeseries(
    db: &Surreal<Client>,
    data: Vec<EvPvLdes>,
    site: &str,
) -> Result<(), surrealdb::Error> {
    // Write data to SurrealDB
    for record in data {
        let temp: Option<EvPvLdes> = db
            .create(("ev_pv_ldes", format!("{}_{}", site, uuid::Uuid::new_v4())))
            .content(record)
            .await?;
        println!("{:?}", temp);
    }

    Ok(())
}

pub async fn battery_storage_to_db(
    batt_system: &mut BatteryStorage,
    db: &Surreal<Client>,
    site: &str,
) -> Result<(), surrealdb::Error> {
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
    let data: Vec<EvPvLdes> = storage_iter
        .zip(input_iter.zip(output_iter.zip(neg_stat_iter)))
        .map(
            |((s_date, s_val), ((i_date, i_val), ((o_date, o_val), (_j_date, j_val))))| {
                assert_eq!(s_date, i_date);
                assert_eq!(s_date, o_date);
                EvPvLdes {
                    date_time: surrealdb::sql::Datetime::from(s_date),
                    storage: s_val,
                    input_power: i_val,
                    output_power: o_val,
                    negative_net_storage: j_val,
                }
            },
        )
        .collect();
    db_update_timeseries(db, data, site).await?;
    Ok(())
}
