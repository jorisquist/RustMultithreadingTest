use std::sync::mpsc::Receiver;
use crate::imu::IMUSample;
use crate::samples::SensorSample;

pub struct StateEstimation {

}


impl StateEstimation {

    pub fn new() -> StateEstimation {
        StateEstimation {}
    }

    pub fn start(self, receiver: Receiver<SensorSample>) {
        loop {
            let sensor_sample = receiver.recv().unwrap();
            match sensor_sample {
                SensorSample::IMU(imu_sample) => {
                    println!("Received {:?}", imu_sample)
                },
                SensorSample::Pressure(pressure_sample) => {
                    println!("Received {:?}", pressure_sample)
                },
                SensorSample::DVL(dvl_sample) => {
                    println!("Received {:?}", dvl_sample)
                },
            }


        }
    }

}