use kombisensor::Kombisensor;


#[derive(Debug)]
pub struct Zone {
    pub kombisensors: Vec<Kombisensor>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ],
        }
    }
}
