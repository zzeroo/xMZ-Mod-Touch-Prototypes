extern crate server;

use std::io::Write;
use server::{Action, Exception, ExceptionType, Server, ShiftRegister, ShiftRegisterType, Kombisensor, Zone};


fn main() {
    let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    let mut relais = ShiftRegister::new(ShiftRegisterType::RELAIS);

    { // Dieser Block ist nötig um den ShiftRegistern eine längere Livetime zu geben
        let mut server = Server::new();
        let mut wartungsintervall = Exception::new(ExceptionType::WartungsIntervall);
        let action_led_wartung_an = Action::new(&led, ShiftRegister::set, 3);
        let action_relais1_aus = Action::new(&relais, ShiftRegister::clear, 1);
        wartungsintervall.actions.push(action_led_wartung_an);
        wartungsintervall.actions.push(action_relais1_aus);
        server.exceptions.push(wartungsintervall);

        let zone1 = Zone::new();
    }


    //
    // server.kombisensors.push(Kombisensor::new());
    //
    // let mut exceptions: Vec<Exception> = vec![];
    //
    // if server.wartungs_intervall() > server.uptime() {
    //     &exceptions.push(Exception::new(ExceptionType::WartungsIntervall));
    // }
    //
    // for kombisensor in server.kombisensors {
    //     if kombisensor.active() == false {
    //         &exceptions.push(Exception::new(ExceptionType::Kabelbruch));
    //     }
    //
    //     for sensor in kombisensor.sensors {
    //         if sensor.direct_value() > sensor.max_direct_value() {
    //             &exceptions.push(Exception::new(ExceptionType::SensorMaxDirectValue));
    //         }
    //
    //         if sensor.average() > sensor.max_average_2() {
    //             &exceptions.push(Exception::new(ExceptionType::SensorMaxAverage2));
    //         }
    //
    //         if sensor.average() > sensor.max_average_1() {
    //             &exceptions.push(Exception::new(ExceptionType::SensorMaxAverage1));
    //         }
    //     }
    // }
}
