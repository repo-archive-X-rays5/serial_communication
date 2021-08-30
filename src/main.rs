use std::{io::Write};
use serialport::{SerialPortType, available_ports};

fn main() {
    match available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("No ports found."),
                1 => println!("Found 1 port:"),
                n => println!("Found {} ports:", n),
            };

            let mut i: u32 = 0;
            for port in ports {
                print!("{}. ", i);
                match port.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("{}: {}", port.port_name, info.product.unwrap())
                    }
                    SerialPortType::BluetoothPort => {
                        println!("{}: Bluetooth", port.port_name);
                    }
                    SerialPortType::PciPort => {
                        println!("{}: PciPort", port.port_name);
                    }
                    SerialPortType::Unknown => {
                        println!("{}: Unknown", port.port_name);
                    }
                }
                i += 1;
            }
        }
        Err(err) => panic!("Failed to get ports: {}", err)
    }
    print!("Port to use (number): ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut port = String::new();
    std::io::stdin().read_line(&mut port).expect("Failed to read port");

    let port: usize = port.trim().parse().expect("Not a number");
    let port = available_ports().unwrap().get(port).unwrap().clone().port_name;

    let mut serial = match serialport::new(port, 9600).open() {
        Ok(val) => val,
        Err(err) => panic!("Failed to start connection: {}", err)
    };
    loop {
        print!("Message to sent: ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut data = String::new();
        std::io::stdin().read_line(&mut data).expect("Failed to read port");

        serial.write_all(data.trim().as_bytes()).expect("Failed to write");
    }
}
