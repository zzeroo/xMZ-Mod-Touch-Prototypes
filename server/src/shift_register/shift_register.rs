
pub enum ShiftRegisterType {
    LED,
    RELAIS,
}

pub struct ShiftRegister {
    shift_register_type: ShiftRegisterType,
}

impl ShiftRegister {
    pub fn new(shift_register_type: ShiftRegisterType) -> Self {
        match shift_register_type {
            ShiftRegisterType::LED      => ShiftRegister { shift_register_type: shift_register_type },
            ShiftRegisterType::RELAIS   => ShiftRegister { shift_register_type: shift_register_type },
        }
    }

    pub fn set(&mut self, num: u32) {

    }

    pub fn clear(&mut self, num: u32) {

    }
}
