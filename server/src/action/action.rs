use shift_register::ShiftRegister;


pub struct Action<'a> (&'a ShiftRegister, fn(&'a mut ShiftRegister, u32), u32);


impl<'a> Action<'a> {
    pub fn new(shift_register: &'a ShiftRegister, func: fn(&mut ShiftRegister, u32), pin: u32) -> Self {
        Action(shift_register, func, pin)
    }
}
