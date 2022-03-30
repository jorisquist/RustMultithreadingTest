mod imu;
mod state_estimation;
mod pressure;
mod samples;
mod dvl;

use clap::Parser;
use std::path::Path;
use std::fs::File;
use std::{fs, thread};
use std::sync::mpsc;

// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to JSON file with the settings
    #[clap(short, long, default_value = "D:\\Repos\\RustMultithreading\\src\\settings.json")]
    settings: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.settings);

    let json = fs::read_to_string(args.settings).unwrap();

    println!("{}", json);

    let parsed = json::parse(&json).unwrap();

    println!("{}", parsed["halldo"]);

    let (tx, rx) = mpsc::channel();
    let pressure_tx = tx.clone();
    let dvl_tx = tx.clone();

    /* IMU */
    let imu_handle = thread::spawn(move || {
        let imu_sensor = imu::IMU::new(1.0);

        imu_sensor.start(tx.clone());
    });

    /* Pressure */
    let pressure_handle = thread::spawn(move || {
        let pressure_sensor = pressure::PressureSensor::new(0.5);

        pressure_sensor.start(pressure_tx);
    });

    /* DVL */
    let dvl_handle = thread::spawn(move || {
        let dvl_client = dvl::DVLClient::new();
        dvl_client.start(dvl_tx);
    });

    /* State Estimation */
    let se_handle = thread::spawn(move || {
        let state_estimation = state_estimation::StateEstimation::new();

        state_estimation.start(rx);
    });

    imu_handle.join();
    se_handle.join();
    pressure_handle.join();
    dvl_handle.join();
}