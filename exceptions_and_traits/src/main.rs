
extern crate exceptions_and_traits;


use exceptions_and_traits::{Action, ShiftRegister, LED, RELAIS, Server, Sensor, SensorCO, SensorNO2, Kombisensor, Exception};


fn main() {
    // Shift Register vorbereiten
    let leds = LED::new();
    let relais = RELAIS::new();

    // Mögliche Aktionen
    let relais_1_aus = Action::new(&relais, RELAIS::clear, 1);
    let led_stoerung_an = Action::new(&leds, LED::set, 2);
    let led_wartung_an = Action::new(&leds, LED::set, 3);

    // Mögliche Ausnahmen
    let wartungsintervall = Exception {
        description: "Wartungsintervall erreicht",
        condition: Server::uptime,
        threshold: 365,
        actions: vec![&led_stoerung_an, &led_wartung_an],
    };
    let kabelbruch = Exception {
        description: "Kabelbruch, Kombisensor nicht erreichbar",
        condition: Kombisensor::active,
        threshold: false,
        actions: vec![&led_stoerung_an, ],
    };
    let zone1_direktwert_no2_ueberschritten = Exception {
        description: "Zone 1: Direktwert NO2 überschritten",
        condition: SensorNO2::direct_value,
        threshold: 15,
        actions: vec![&led_stoerung_an, ],
    };


    // Serverobjekt erstellen und Kombisensoren erzeugen
    let mut server = Server::new();
    for i in 0..4 {
        server.kombisensors.push(Kombisensor::new());
    }


    // Main Loop
    loop {
        // Wartungsintervall prüfen
        wartungsintervall.evaluete(&server);
        println!();

        // Kabelbruch prüfen
        for kombisensor in &server.kombisensors {
            kabelbruch.evaluete(&kombisensor);
        }
        println!();

        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
