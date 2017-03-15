use shift_register::ShiftRegister;


#[derive(Debug)]
pub struct RELAIS {
    data: u64,
}

impl RELAIS {
    pub fn new() -> Self {
        RELAIS {
            data: 0,
        }
    }
}

impl ShiftRegister for RELAIS {
    fn set(&mut self, num: u64) {
        self.data |= 1 << num -1;
        println!("Setze RELAIS {}", num);
    }

    fn clear(&mut self, num: u64) {
        self.data &= !(1 << num - 1);
        println!("Clear RELAIS {}", num);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let relais = RELAIS::new();

        assert_eq!(relais.data, 0);
    }

    #[test]
    fn test_set() {
        let mut relais = RELAIS::new();

        assert_eq!(relais.data, 0);
        relais.set(1);
        assert_eq!(relais.data, 0b1);
    }

    #[test]
    fn test_clear() {
        let mut relais = RELAIS::new();
        relais.set(1);
        assert_eq!(relais.data, 0b1);
        relais.clear(1);
        assert_eq!(relais.data, 0);
    }
}
