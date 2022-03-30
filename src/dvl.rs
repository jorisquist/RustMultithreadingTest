use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use crate::samples::SensorSample;

pub struct PressureSensor {
    frequency: f32,

}

#[derive(Debug)]
pub struct PressureSample {
    pressure: f32
}

impl PressureSensor {
    pub fn new(frequency: f32) -> PressureSensor {
        return PressureSensor { frequency };
    }

    pub fn start(self, sender: Sender<SensorSample>) {
        loop {
            let sample = PressureSample { pressure: rand::random() };

            println!("Sending: {:?}", sample);

            sender.send(SensorSample::Pressure(sample));

            sleep(Duration::from_millis((1000.0/self.frequency) as u64));
        }
    }
}

