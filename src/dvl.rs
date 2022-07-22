use crossbeam::Sender;
// use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use crate::samples::SensorSample;

pub struct DVLClient {

}

#[derive(Debug)]
pub struct DVLSample {
    velocity: [f32; 3]
}

impl DVLClient {
    pub fn new() -> DVLClient {
        return DVLClient {};
    }

    pub fn start(self, sender: Sender<SensorSample>) {
        loop {
            let sample = DVLSample { velocity: [rand::random(), rand::random(), rand::random()] };

            sender.send(SensorSample::DVL(sample));

            sleep(Duration::from_millis((3000) as u64));
        }
    }
}

