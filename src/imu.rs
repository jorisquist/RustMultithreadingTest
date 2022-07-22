use std::mem;
use crossbeam::Sender;
// use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use nalgebra::{ArrayStorage, vector};
use nalgebra::Vector3;

use hdf5::{H5Type};

use serde::{Deserialize, Serialize};

use crate::samples::SensorSample;

pub struct IMU {
    frequency: f32,
}

#[derive(Debug, Serialize, Deserialize, H5Type, Clone, PartialEq, Copy)]
#[repr(C)]
pub struct IMUSample {
    pub(crate) acc: [f32; 3],
    pub(crate) mag: [f32; 3],
    pub(crate) gyro: [f32; 3],
}

impl IMU {
    pub fn new(frequency: f32) -> IMU {
        return IMU { frequency };
    }

    pub fn start(self, sender: Sender<SensorSample>) {
        loop {
            let sample = IMUSample {
                acc: [rand::random(), rand::random(), rand::random()],
                mag: [rand::random(), rand::random(), rand::random()],
                gyro: [rand::random(), rand::random(), rand::random()],
            };
            // sample.acc.data.
            // let test: tuple<f32>;

            let encoded: Vec<u8> = bincode::serialize(&sample).unwrap();

            print!("{:?}", encoded.len());

            sender.send(SensorSample::IMU(sample));

            sleep(Duration::from_millis((1000.0 / self.frequency) as u64));
        }
    }
}

