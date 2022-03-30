use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use crate::samples::SensorSample;

pub struct IMU {
    frequency: f32,

}

#[derive(Debug)]
pub struct IMUSample {
    acc: [f32; 3],
    mag: [f32; 3],
    gyro: [f32; 3],
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

            sender.send(SensorSample::IMU(sample));

            sleep(Duration::from_millis((1000.0/self.frequency) as u64));
        }
    }
}

