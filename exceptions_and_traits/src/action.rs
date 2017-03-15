/// `Actions` werden `Exceptions` zugewiesen
///
/// Wird eine `Exception` ausgewertet werden allen mit ihr verbundenen `Action` ausgeführt.
use shift_register::ShiftRegister;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Action<'a, T, P>
    where   T: 'a + ShiftRegister,
            P: fmt::Debug,
{
    pub shift_register: &'a T,
    pub action: fn(&mut T, u64),
    pub pin: P,
}

impl<'a, T, P> Action<'a, T, P>
    where   T: 'a + ShiftRegister + Debug,
            P: fmt::Debug,
{
    pub fn new(shift_register: &'a T, action: fn(&mut T, u64), pin: P) -> Self {
        Action {
            shift_register: shift_register,
            action: action,
            pin: pin,
        }
    }
}

impl<'a, T, P> Debug for Action<'a, T, P>
    where   T: 'a + ShiftRegister + Debug,
            P: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Action {{ shift_register: {:?}, action: {:?}, pin: {:?} }}", self.shift_register, "", self.pin)
    }
}



#[cfg(test)]
mod tests {
    use action::Action;
    use shift_register::ShiftRegister;
    use led::LED;

    #[test]
    fn test_new() {
        let led = LED::new();
        let action = Action::new(&led, LED::set, 1);
    }
}
