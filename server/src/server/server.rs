use kombisensor::Kombisensor;
use time;

pub struct Server {
    pub startup_time: time::Tm,
    pub kombisensors: Vec<Kombisensor>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            startup_time: time::now(),
            kombisensors: vec![],
        }
    }

    pub fn uptime(&self) -> time::Duration {
        time::now() - self.startup_time
    }

    pub fn wartungs_intervall(&self) -> time::Duration {
        // time::Duration::days(365)
        time::Duration::seconds(10)
    }
}
