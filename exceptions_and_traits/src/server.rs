use exception::Exceptional;
use kombisensor::Kombisensor;


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Server {
    pub kombisensors: Vec<Kombisensor>,
}

impl Exceptional for Server {}

impl Server {
    pub fn new() -> Self {
        Server {
            kombisensors: vec![],
        }
    }

    pub fn uptime(&self) -> u32 {
        365
    }
}
