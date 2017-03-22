use exception::Exception;
use time;
use zone::Zone;


pub struct Server<'a> {
    pub startup_time: time::Tm,
    pub exceptions: Vec<Exception<'a>>,
    pub zones: Vec<Zone>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            startup_time: time::now(),
            exceptions: vec![],
            zones: vec![],
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
