mod imu;
mod state_estimation;
mod pressure;
mod samples;
mod dvl;
mod pressure_calibrator;
mod state_logger;

use clap::Parser;
use std::path::Path;
use std::{fs, thread};
use std::thread::sleep;
use std::time::Duration;
use hdf5::{File, Result};
use ndarray::{Array, Array1, Array2};
use ndarray::{arr2, s};
use crate::imu::IMUSample;
use crate::samples::SensorSample::IMU;
use crate::state_logger::StateLogger;

// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to JSON file with the settings
    #[clap(short, long, default_value = "src/settings.json")]
    settings: String,
}

fn main() -> Result<()> {
    // let args = Args::parse();
    //
    // println!("{}", args.settings);
    //
    // let json = fs::read_to_string(args.settings).unwrap();
    //
    // println!("{}", json);
    //
    // let parsed = json::parse(&json).unwrap();
    //
    // println!("{}", parsed["hallo"]);
    //
    // let (tx, rx) = crossbeam::channel::unbounded();
    // let pressure_tx = tx.clone();
    // let dvl_tx = tx.clone();
    // let se_rx = rx.clone();
    //
    // /* IMU */
    // let imu_handle = thread::spawn(move || {
    //     let imu_sensor = imu::IMU::new(1.0);
    //
    //     imu_sensor.start(tx.clone());
    // });
    //
    // /* Pressure */
    // let pressure_handle = thread::spawn(move || {
    //     let pressure_sensor = pressure::PressureSensor::new(0.5);
    //
    //     pressure_sensor.start(pressure_tx);
    // });
    //
    // /* DVL */
    // let dvl_handle = thread::spawn(move || {
    //     let dvl_client = dvl::DVLClient::new();
    //     dvl_client.start(dvl_tx);
    // });
    //
    // /* State Estimation */
    // let se_handle = thread::spawn(move || {
    //     let state_estimation = state_estimation::StateEstimation::new();
    //
    //     state_estimation.start(se_rx);
    // });
    //
    // let pc_handle = thread::spawn(move || {
    //     let pressure_calibrator = pressure_calibrator::PressureCalibrator::new();
    //
    //     pressure_calibrator.start(rx);
    // });
    //
    // imu_handle.join();
    // se_handle.join();
    // pressure_handle.join();
    // dvl_handle.join();


    let file = File::create("chunking.h5")?;

    // let (ny, nx) = (100, 100);
    // let arr = Array2::from_shape_fn((ny, nx), |(j, i)| (1000 * j + i) as f32);

    // println!("{} {:?}", arr, arr.shape());

    let state_logger = StateLogger::new(&file, String::from("imu"));
    let sample = IMUSample {
        acc: [rand::random(), rand::random(), rand::random()],
        mag: [rand::random(), rand::random(), rand::random()],
        gyro: [rand::random(), rand::random(), rand::random()],
    };

    println!("{:?}", sample);
    
    // writing one chunk at a time is the most efficient
    state_logger.log(sample);

    println!("{:?}", state_logger.read(0));

    file.close()?;
    Ok(())

}