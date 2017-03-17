use sensor::Sensor;

pub struct Kombisensor {
    pub sensors: Vec<Sensor>,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            sensors: vec![],
        }
    }

    pub fn active(&self) -> bool {
        false
    }
}
