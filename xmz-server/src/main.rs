extern crate xmz_server;

use xmz_server::{ HasException, Kombisensor, Server, ServerError, Zone};



fn server_update(server: Server) -> Result<(), ServerError> {
    server.check_exceptions();

    for mut zone in server.zones {
        for mut kombisensor in zone.kombisensors {

            kombisensor.check_exceptions();

            for mut sensor in kombisensor.sensors {

                sensor.check_exceptions();

                sensor.update();
            }
        }
    }

    Ok(())
}

fn server_init() -> Result<Server, ServerError> {
    let server = Server::new();

    // Server Startkonfiguration hier ...

    Ok(server)
}

fn main() {
    match server_init() {
        Ok(server) => {
            println!("{:#?}", &server);

            server_update(server);
        }
        Err(e) => {
            println!("{}", e);
            ::std::process::exit(1);
        }
    }
}
