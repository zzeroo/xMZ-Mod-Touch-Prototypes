use exception::Exceptional;


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Kombisensor;

impl Exceptional for Kombisensor {}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {

        }
    }

    pub fn active(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    use kombisensor::Kombisensor;
    use exception::{Exceptional, Exception};
    use shift_register::ShiftRegister;
    use action::Action;
    use led::LED;

    #[test]
    fn test_new() {
        let kombisensor = Kombisensor::new();
    }

    #[test]
    fn test_kabelburch() {
        let kombisensor = Kombisensor::new();
        let led = LED::new();
        let action = Action::new(&led, LED::set, 1);

        let kabelbruch = Exception {
            description: "Kabelbruch, Kombisensor nicht erreichbar",
            condition: Kombisensor::active,
            threshold: false,
            actions: vec![&action],
        };

        assert_eq!(kabelbruch.is_reached(&kombisensor), true);
    }
}
