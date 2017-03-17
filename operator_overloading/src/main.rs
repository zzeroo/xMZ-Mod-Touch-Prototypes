use std::fmt;

trait Sensor: Sized {
    fn concentration(&self) -> u64;
    fn direct_value(&self) -> u64;
}
impl<S> Sensor for Box<S>
    where   S: Sensor + Sized,
{
    fn concentration(&self) -> u64 { 0 }
    fn direct_value(&self) -> u64 {
        (**self).concentration()
    }
}

struct SensorCO {
    adc_value: u64,
}
impl fmt::Display for SensorCO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CO Messzelle")
    }
}
impl SensorCO {
    fn new() -> Self {
        SensorCO {
            adc_value: 0,
        }
    }
}
impl Sensor for SensorCO {
    fn concentration(&self) -> u64 {
        self.adc_value + 900
    }
    fn direct_value(&self) -> u64 {
        self.concentration()
    }
}

struct SensorNO2 {
    adc_value: u64,
}
impl fmt::Display for SensorNO2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NO2 Messzelle")
    }
}
impl SensorNO2 {
    fn new() -> Self {
        SensorNO2 {
            adc_value: 0,
        }
    }
}
impl Sensor for SensorNO2 {
    fn concentration(&self) -> u64 {
        self.adc_value + 332
    }
    fn direct_value(&self) -> u64 {
        self.concentration()
    }
}

struct Kombisensor<S>
    where   S: Sensor + Sized,
{
    sensors: Vec<Box<S>>,
}
impl<S> fmt::Display for Kombisensor<S>
    where   S: Sensor + Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CO/ NO2 Kombisensor")
    }
}
impl<S> Kombisensor<S>
    where   S: Sensor + Sized,
{
    fn new() -> Self {
        Kombisensor {
            sensors: vec![
                Box::new(SensorNO2::new()) as Box<Sensor>,
                Box::new(SensorCO::new()) as Box<Sensor>,
            ],
        }
    }
}

struct Server<S>
    where   S: Sensor + Sized,
{
    kombisensors: Vec<Kombisensor<S>>,
}
impl<S> Server<S>
    where   S: Sensor + Sized,
{
    fn new() -> Self {
        Server {
            kombisensors: vec![
                Kombisensor::new(),
                Kombisensor::new(),
            ],
        }
    }
}


fn main() {
    let server = Server::new();

    for kombisensor in &server.kombisensors {
        println!("{}", kombisensor);

        for sensor in &kombisensor.sensors {
            println!("{}", sensor);
            println!("{}", sensor.concentration());
        }
    }
}
