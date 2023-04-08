use configparser::ini::Ini;
use std::path::Path;

fn main() {
    // Create a new `Ini` object
    let mut config = Ini::new();

    // Load a configuration file
    let path = Path::new("config.ini");
    match config.load(&path) {
        Ok(_) => println!("Configuration loaded successfully!"),
        Err(e) => println!("Error loading configuration: {}", e),
    }

    // Get a value from the configuration file
    let value = config.get("SERVER", "port");

    match value {
        Some(val) => println!("Server Port: {}", val),
        None => println!("Value not found."),
    }

    let com_port = config.get("SERIAL", "port");

    match com_port {
        Some(val) => println!("Serial Port: {}", val),
        None => println!("Value not found."),
    }

    let baud_rate = config.get("SERIAL", "baud");

    match baud_rate {
        Some(val) => println!("Baud rate: {}", val),
        None => println!("Value not found."),
    }

}
