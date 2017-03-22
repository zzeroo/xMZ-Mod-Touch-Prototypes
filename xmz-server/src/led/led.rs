use shift_register::ShiftRegister;

#[derive(Debug)]
pub struct Led {
    data: u64,
}

impl Led {
    pub fn new() -> Self {
        Led {
            data: 0,
        }
    }
}

impl ShiftRegister for Led {
    fn set(&mut self, num: u64) {
        println!("Setze LED Nummer: {}", num);
    }

    fn clear(&mut self, num: u64) {
        println!("Lösche LED Nummer: {}", num);
    }
}
