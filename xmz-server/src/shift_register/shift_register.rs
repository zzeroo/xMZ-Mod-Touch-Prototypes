

pub trait ShiftRegister {
    fn set(&mut self, num: u64);

    fn clear(&mut self, num: u64);
}
