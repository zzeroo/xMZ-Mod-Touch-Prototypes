extern crate server;

use std::io::Write;
use server::{Action, Exception, ExceptionType, Server, ShiftRegister, ShiftRegisterType, Kombisensor};


fn main() {
    let mut server = Server::new();

    let mut led = ShiftRegister::new(ShiftRegisterType::LED);
    let mut relais = ShiftRegister::new(ShiftRegisterType::RELAIS);

    let mut wartungsintervall = Exception::new(ExceptionType::WartungsIntervall);
    let action_led_wartung_an = Action::new(&led, ShiftRegister::set, 3);
    let action_relais1_aus = Action::new(&relais, ShiftRegister::clear, 1);
    wartungsintervall.actions.push(action_led_wartung_an);
    wartungsintervall.actions.push(action_relais1_aus);
    server.exceptions.push(wartungsintervall);


    loop {
        print!(".");
        std::io::stdout().flush();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // for exception in exceptions {
    //     exception.actions();
    // }


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
