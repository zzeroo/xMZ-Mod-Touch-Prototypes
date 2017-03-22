use led::Led;
use relais::Relais;
use exception::HasException;
use zone::Zone;


#[derive(Debug)]
pub struct Server {
    leds: Led,
    relais: Relais,
    // exceptions: Vec<Box<Exception>>,
    pub zones: Vec<Zone>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            leds: Led::new(),
            relais: Relais::new(),
            zones: vec![
                Zone::new(),
                Zone::new(),
            ],
        }
    }
}

impl HasException for Server {
    fn check_exceptions(&self) {
        println!("Prüfe Ausnahmen (exceptions) am Server");
    }
}
