use shift_register::ShiftRegister;

#[derive(Debug)]
pub struct Relais {
    data: u64,
}

impl Relais {
    pub fn new() -> Self {
        Relais {
            data: 0,
        }
    }
}

impl ShiftRegister for Relais {
    fn set(&mut self, num: u64) {
        println!("Setze RELAIS Nummer: {}", num);
    }

    fn clear(&mut self, num: u64) {
        println!("Lösche RELAIS Nummer: {}", num);
    }
}
