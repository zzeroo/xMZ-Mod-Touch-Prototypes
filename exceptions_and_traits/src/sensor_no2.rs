use sensor::Sensor;
use exception::Exceptional;

impl Exceptional for SensorNO2 {}

pub struct SensorNO2 {
    concentration: u64,
}

impl SensorNO2 {
    pub fn new() -> Self {
        SensorNO2 {
            concentration: 0,
        }
    }
}


impl Sensor for SensorNO2 {
    fn direct_value(&self) -> u64 {
        self.concentration
    }
}
