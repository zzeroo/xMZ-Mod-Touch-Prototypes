use shift_register::ShiftRegister;
use std::marker::PhantomData;
use server::Server;
use action::Action;
use std::fmt::Debug;


/// Marker trait
///
/// Damit werden Datentypen markiert die Ausnahmen werfen können.
/// Zum Beispiel der Server, Kombisensor und/ oder Sensor sind `Exceptional`.
pub trait Exceptional {}

/// Mögliche Ausnahmen
///
/// Eine Ausnahme besteht aus einer Beschreibung `description`, einer Bedingung `condition`,
/// einem Schwellwert `threshold` und einer Sammlung möglicher Aktionen `actions`.
pub struct Exception<'a, S, T, E, P>
    where   S: 'a + ShiftRegister + Debug,
            T: 'a + Debug,
            E: Exceptional,
            P: PartialEq,
{
    pub description: &'a str,
    pub condition: fn(&E) -> P,
    pub threshold: P,
    pub actions: Vec<&'a Action<'a, S, T>>,
}

impl<'a, S, T, E, P> Exception<'a, S, T, E, P>
    where   S: 'a + ShiftRegister + Debug,
            T: 'a + Debug,
            E: Exceptional,
            P: PartialEq,
{
    pub fn new(description: &'a str, condition: fn(&E) -> P, threshold: P) -> Self {
        Exception {
            description: description,
            condition: condition,
            threshold: threshold,
            actions: vec![],
        }
    }

    pub fn is_reached(&self, exceptional: &E) -> bool {
        (self.condition)(exceptional) == self.threshold
    }

    pub fn evaluete(&self, exceptional: &E) {
        if self.is_reached(exceptional) {
            println!("Ausnahme: {}", self.description);
            for action in &self.actions {
                println!("\tAction fired: {:?}", action);
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use server::Server;
    use exception::{Exceptional, Exception};
    use led::LED;
    use shift_register::ShiftRegister;
    use action::Action;

    // #[test]
    // fn test_new() {
    //     let wartungsintervall = Exception::new("Wartungsintervall erreicht", Server::uptime, 356);
    // }

    #[test]
    fn test_manuel() {
        let led = LED::new();
        let action = Action::new(&led, LED::set, 1);

        let wartungsintervall = Exception {
            description: "Wartungsintervall erreicht",
            condition: Server::uptime,
            threshold: 365,
            actions: vec![&action],
        };
    }

    // #[test]
    // fn test_is_reached() {
    //     let server = Server::new();
    //
    //     let wartungsintervall = Exception {
    //         description: "Wartungsintervall erreicht",
    //         condition: Server::uptime,
    //         threshold: 356,
    //     };
    //
    //     assert_eq!(wartungsintervall.is_reached(&server), true);
    //
    // }
}
