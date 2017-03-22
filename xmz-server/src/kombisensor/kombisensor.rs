use exception::HasException;
use sensor_co::SensorCO;
use sensor_no2::SensorNO2;
use sensor::Sensor;


#[derive(Debug)]
pub struct Kombisensor {
    pub sensors: Vec<Box<Sensor>>,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Box::new(SensorNO2::new()),
                Box::new(SensorCO::new()),
            ],
        }
    }
}

impl HasException for Kombisensor {
    fn check_exceptions(&self) {
        println!("Prüfe Ausnahmen (exceptions) am Kombisensor");
    }
}
