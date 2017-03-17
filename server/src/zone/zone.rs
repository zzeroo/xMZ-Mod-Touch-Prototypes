use kombisensor::Kombisensor;

pub struct Zone {
    pub kombisensors: Vec<Kombisensor>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            kombisensors: vec![],
        }
    }
}
