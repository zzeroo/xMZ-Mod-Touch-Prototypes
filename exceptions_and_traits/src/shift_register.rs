pub trait ShiftRegister {
    fn set(&mut self, num: u64);
    fn clear(&mut self, num: u64);
}

impl<S: ?Sized> ShiftRegister for Box<S>
    where S: ShiftRegister
{
    fn set(&mut self, num: u64) { (**self).set(num) }
    fn clear(&mut self, num: u64) { (**self).clear(num) }
}
