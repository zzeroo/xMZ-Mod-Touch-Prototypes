use sensor::Sensor;
use exception::Exceptional;

impl Exceptional for SensorCO {}

pub struct SensorCO {
    concentration: u64,
}

impl SensorCO {
    pub fn new() -> Self {
        SensorCO {
            concentration: 0,
        }
    }
}


impl Sensor for SensorCO {
    fn direct_value(&self) -> u64 {
        self.concentration
    }
}
