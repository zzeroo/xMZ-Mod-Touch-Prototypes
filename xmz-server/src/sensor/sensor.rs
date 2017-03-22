use exception::HasException;
use std::fmt;

pub trait Sensor: fmt::Debug {
    fn update(&mut self);
}


impl HasException for Sensor {
    fn check_exceptions(&self) {
        println!("Prüfe Ausnahmen (exceptions) am Sensor");
    }
}
