use shift_register::ShiftRegister;


#[derive(Debug)]
pub struct LED {
    data: u64,
}

impl LED {
    pub fn new() -> Self {
        LED {
            data: 0,
        }
    }
}

impl ShiftRegister for LED {
    fn set(&mut self, num: u64) {
        self.data |= 1 << num -1;
        println!("Setze LED {}", num);
    }

    fn clear(&mut self, num: u64) {
        self.data &= !(1 << num - 1);
        println!("Clear LED {}", num);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let led = LED::new();

        assert_eq!(led.data, 0);
    }

    #[test]
    fn test_set() {
        let mut led = LED::new();

        assert_eq!(led.data, 0);
        led.set(1);
        assert_eq!(led.data, 0b1);
    }

    #[test]
    fn test_clear() {
        let mut led = LED::new();
        led.set(1);
        assert_eq!(led.data, 0b1);
        led.clear(1);
        assert_eq!(led.data, 0);
    }
}
