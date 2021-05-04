use std::time::Duration;

use thelio_io::Io;

fn main() {
    for port_info in serialport::available_ports().unwrap() {
        println!("{:?}", port_info);
        match port_info.port_type {
            serialport::SerialPortType::UsbPort(usb_info) => {
                if usb_info.vid == 0x1209 && usb_info.pid == 0x1776 {
                    println!("Thelio Io at {}", port_info.port_name);

                    let port = serialport::new(port_info.port_name, 115200)
                        .timeout(Duration::from_millis(1))
                        .open()
                        .unwrap();

                    let mut io = Io::new(port, 1000);

                    println!("reset: {:?}", io.reset());

                    println!("revision: {:?}", io.revision());
                    println!("suspend: {:?}", io.suspend());
                    println!("set_suspend: {:?}", io.set_suspend(1));
                    println!("suspend: {:?}", io.suspend());

                    for device in &["CPUF", "INTF", "POWB"] {
                        println!("device: {}", device);
                        println!("  tach: {:?}", io.tach(device));
                        println!("  duty: {:?}", io.duty(device));
                        println!("  set_duty: {:?}", io.set_duty(device, 0));
                        println!("  duty: {:?}", io.duty(device));
                    }
                }
            },
            _ => (),
        }
    }
}
