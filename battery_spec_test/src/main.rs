use crate::ev_chargers::Charger;
use crate::photovoltaic::pv_base_system::PvSystem;
use battery_spec_test::energy_components::batteries::BatteryStorage;
use battery_spec_test::energy_components::*;
use battery_spec_test::setup_and_run_simulation;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

type Lambdas = Vec<f64>;

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    // Sign in as the root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    // Setup SurrealDB with namespaces, databases, and tables
    db.use_ns("charging-station").await.unwrap();
    db.use_db("batteries").await.unwrap();

    let mut template_charger_180kw = Charger::new(
        180000.0, 150.0, 1000.0, 600.0, 480.0, 60, 240.0, 300, 199.3, 0.98, 0.94,
    );
    let solar_system: PvSystem =
        PvSystem::new(13000.0, 450.0, 0.64, 7.0, 7.0, "full_irradiance_data.csv");
    let mut battery_storage = BatteryStorage::new(2000000.0, 4.0, 80.0, 48.0, 90.0);
    let start_date = "2024-01-01 00:00:00+0000";
    let end_date = "2024-03-30 00:00:00+0000";

    let battery_param_vec: Vec<(f32, f32)> = vec![
        (2500000.0, 4.0),
        (3000000.0, 4.0),
        (3500000.0, 4.0),
        (3500000.0, 4.0),
    ];
    let pv_param_vec: Vec<(usize, usize, f32)> = vec![
        (22, 500, 450.0),
        (24, 500, 450.0),
        (28, 500, 450.0),
        (32, 500, 450.0),
    ];
    let ev_charger_param_vec: Vec<usize> = vec![20];
    let lamb_vec: Vec<Lambdas> = vec![
        vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 45.0, 31.0, 31.0, 1.0, 45.0, 45.0, 1.0, 45.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 45.0, 31.0, 31.0, 1.0, 45.0, 45.0, 1.0, 45.0, 1.0,
            1.0, 20.0, 20.0, 20.0, 5.0, 1.0, 1.0, 1.0,
        ],
    ];

    setup_and_run_simulation(
        &db,
        &mut template_charger_180kw,
        solar_system,
        &mut battery_storage,
        start_date,
        end_date,
        battery_param_vec,
        pv_param_vec,
        ev_charger_param_vec,
        lamb_vec,
    )
    .await
    .unwrap();
}
