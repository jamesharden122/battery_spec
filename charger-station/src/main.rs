#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use chrono::NaiveDate;
use dioxus::prelude::*;

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug)]
pub struct BatteryStorage {
    pub capacity: f32,
    pub watt_hours: f32,
    pub depth_of_discharge: f32,
    pub battery_system_voltage: f32,
    pub efficiency: f32,
}

#[derive(Clone, Debug)]
pub struct PvSystem {
    pub num_panels: f32,
    pub panel_watts: f32,
    pub panel_cost: f32,
    pub transformer_efficiency: f32,
    pub panel_width: f32,
    pub panel_length: f32,
}

#[component]
pub fn ConfigForm() -> Element {
    let charger = use_state(|| Charger {
        maximum_power_w: 180000.0,
        output_voltage_min_vdc: 150.0,
        output_voltage_max_vdc: 1000.0,
        max_output_current_a: 600.0,
        input_voltage_vac: 480.0,
        input_frequency_hz: 60,
        fla_a: 240.0,
        breaker_rating_a: 300,
        rated_power_kva: 199.3,
        power_factor_at_full_load: 0.98,
        efficiency_at_nominal_power: 0.94,
    });

    let pv_system = use_state(|| PvSystem {
        num_panels: 13000.0,
        panel_watts: 450.0,
        panel_cost: 0.64,
        transformer_efficiency: 7.0,
        panel_width: 7.0,
        panel_length: 7.0,
    });

    let battery_storage = use_state(|| BatteryStorage {
        capacity: 2000000.0,
        watt_hours: 4.0,
        depth_of_discharge: 80.0,
        battery_system_voltage: 48.0,
        efficiency: 90.0,
    });

    let start_date = use_state(|| "2024-01-01".to_string());
    let end_date = use_state(|| "2024-03-30".to_string());

    rsx!(
        div {
            h2 { "Charger Configuration" }
            form {
                div {
                    label { "Maximum Power (W):" }
                    input {
                        r#type: "number",
                        value: "{charger.maximum_power_w}",
                        oninput: move |e| charger.maximum_power_w = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Output Voltage Min (VDC):" }
                    input {
                        r#type: "number",
                        value: "{charger.output_voltage_min_vdc}",
                        oninput: move |e| charger.output_voltage_min_vdc = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Output Voltage Max (VDC):" }
                    input {
                        r#type: "number",
                        value: "{charger.output_voltage_max_vdc}",
                        oninput: move |e| charger.output_voltage_max_vdc = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Max Output Current (A):" }
                    input {
                        r#type: "number",
                        value: "{charger.max_output_current_a}",
                        oninput: move |e| charger.max_output_current_a = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Input Voltage (VAC):" }
                    input {
                        r#type: "number",
                        value: "{charger.input_voltage_vac}",
                        oninput: move |e| charger.input_voltage_vac = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Input Frequency (Hz):" }
                    input {
                        r#type: "number",
                        value: "{charger.input_frequency_hz}",
                        oninput: move |e| charger.input_frequency_hz = e.value.parse().unwrap_or(0),
                    }
                }
                div {
                    label { "FLA (A):" }
                    input {
                        r#type: "number",
                        value: "{charger.fla_a}",
                        oninput: move |e| charger.fla_a = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Breaker Rating (A):" }
                    input {
                        r#type: "number",
                        value: "{charger.breaker_rating_a}",
                        oninput: move |e| charger.breaker_rating_a = e.value.parse().unwrap_or(0),
                    }
                }
                div {
                    label { "Rated Power (KVA):" }
                    input {
                        r#type: "number",
                        value: "{charger.rated_power_kva}",
                        oninput: move |e| charger.rated_power_kva = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Power Factor at Full Load:" }
                    input {
                        r#type: "number",
                        value: "{charger.power_factor_at_full_load}",
                        oninput: move |e| charger.power_factor_at_full_load = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Efficiency at Nominal Power:" }
                    input {
                        r#type: "number",
                        value: "{charger.efficiency_at_nominal_power}",
                        oninput: move |e| charger.efficiency_at_nominal_power = e.value.parse().unwrap_or(0.0),
                    }
                }
            }

            h2 { "PV System Configuration" }
            form {
                div {
                    label { "Number of Panels:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.num_panels}",
                        oninput: move |e| pv_system.num_panels = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Panel Watts:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.panel_watts}",
                        oninput: move |e| pv_system.panel_watts = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Panel Cost:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.panel_cost}",
                        oninput: move |e| pv_system.panel_cost = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Transformer Efficiency:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.transformer_efficiency}",
                        oninput: move |e| pv_system.transformer_efficiency = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Panel Width:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.panel_width}",
                        oninput: move |e| pv_system.panel_width = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Panel Length:" }
                    input {
                        r#type: "number",
                        value: "{pv_system.panel_length}",
                        oninput: move |e| pv_system.panel_length = e.value.parse().unwrap_or(0.0),
                    }
                }
            }

            h2 { "Battery Storage Configuration" }
            form {
                div {
                    label { "Capacity (Wh):" }
                    input {
                        r#type: "number",
                        value: "{battery_storage.capacity}",
                        oninput: move |e| battery_storage.capacity = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Watt Hours:" }
                    input {
                        r#type: "number",
                        value: "{battery_storage.watt_hours}",
                        oninput: move |e| battery_storage.watt_hours = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Depth of Discharge:" }
                    input {
                        r#type: "number",
                        value: "{battery_storage.depth_of_discharge}",
                        oninput: move |e| battery_storage.depth_of_discharge = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Battery System Voltage:" }
                    input {
                        r#type: "number",
                        value: "{battery_storage.battery_system_voltage}",
                        oninput: move |e| battery_storage.battery_system_voltage = e.value.parse().unwrap_or(0.0),
                    }
                }
                div {
                    label { "Efficiency:" }
                    input {
                        r#type: "number",
                        value: "{battery_storage.efficiency}",
                        oninput: move |e| battery_storage.efficiency = e.value.parse().unwrap_or(0.0),
                    }
                }
            }

            h2 { "Simulation Date Range" }
            form {
                div {
                    label { "Start Date:" }
                    input {
                        r#type: "date",
                        value: "{start_date}",
                        oninput: move |e| start_date.set(e.value.clone()),
                    }
                }
                div {
                    label { "End Date:" }
                    input {
                        r#type: "date",
                        value: "{end_date}",
                        oninput: move |e| end_date.set(e.value.clone()),
                    }
                }
            }

            button {
                onclick: move |_| {
                    // Handle form submission, validation, or data processing here
                    println!("Configuration saved");
                },
                "Save Configuration"
            }
        }
    )
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
