use sensor::Sensor;
use exception::HasException;


#[derive(Debug)]
pub struct SensorNO2;

impl SensorNO2 {
    pub fn new() -> Self {
        SensorNO2 {

        }
    }
}

impl Sensor for SensorNO2 {
    fn update(&mut self) {
        println!("Update NO2 Sensor");
    }
}

impl HasException for SensorNO2 {
    fn check_exceptions(&self) {
        println!("Prüfe Ausnahmen (exceptions) am Sensor NO2");
    }
}
