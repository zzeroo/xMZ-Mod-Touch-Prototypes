extern crate server;

use std::io::Write;
use server::{Exception, ExceptionType, Server, Kombisensor};


fn main() {
    let mut server = Server::new();

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
