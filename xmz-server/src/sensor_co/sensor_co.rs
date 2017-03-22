use exception::HasException;
use sensor::Sensor;


#[derive(Debug)]
pub struct SensorCO;

impl SensorCO {
    pub fn new() -> Self {
        SensorCO {

        }
    }
}

impl Sensor for SensorCO {
    fn update(&mut self) {
        println!("Update CO Sensor");
    }
}

impl HasException for SensorCO {
    fn check_exceptions(&self) {
        println!("Prüfe Ausnahmen (exceptions) am Sensor CO");
    }
}
