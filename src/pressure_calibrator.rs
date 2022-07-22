use crossbeam::Receiver;
use crate::pressure::PressureSample;
use crate::samples::SensorSample;

pub struct PressureCalibrator {}


impl PressureCalibrator {
    pub fn new() -> PressureCalibrator {
        PressureCalibrator {}
    }

    pub fn start(self, receiver: Receiver<SensorSample>) {
        let mut i = 0;
        while i < 10 {

            match receiver.recv().unwrap() {
                SensorSample::Pressure(pressure_sample) => {
                    i += 1;
                    println!("Received pressure {:?}", pressure_sample)
                }
                _ => {}
            }

        }
    }
}