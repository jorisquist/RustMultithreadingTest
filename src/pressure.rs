use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

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

    pub fn start(self, sender: Sender<IMUSample>) {
        loop {
            let sample = IMUSample {
                acc: [rand::random(), rand::random(), rand::random()],
                mag: [rand::random(), rand::random(), rand::random()],
                gyro: [rand::random(), rand::random(), rand::random()],
            };

            println!("Sending: {:?}", sample);

            sender.send(sample);

            sleep(Duration::from_secs(1));
        }
    }
}

