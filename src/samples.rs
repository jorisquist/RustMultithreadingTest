use crate::dvl::DVLSample;
use crate::imu::IMUSample;
use crate::pressure::PressureSample;

pub enum SensorSample {
    IMU(IMUSample),
    Pressure(PressureSample),
    DVL(DVLSample)
}