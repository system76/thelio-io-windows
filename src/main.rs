use std::{
    io::{
        BufRead,
        BufReader,
        Write,
    },
    process::{
        Command,
        Stdio,
    },
    thread::sleep,
    time::Duration,
};

use thelio_io::Io;

fn main() {
    let smbios = smbioslib::table_load_from_device().unwrap();

    let sys_vendor = smbios.find_map(
        |sys: smbioslib::SMBiosSystemInformation| sys.manufacturer()
    ).unwrap();

    let product_version = smbios.find_map(
        |sys: smbioslib::SMBiosSystemInformation| sys.version()
    ).unwrap();

    match (sys_vendor.as_str(), product_version.as_str()) {
        ("System76", "thelio-mira-r1") => println!("System76 Thelio Mira (thelio-mira-r1)"),
        _ => panic!("unsupported sys_vendor '{}' and product_version '{}'", sys_vendor, product_version),
    }

    let mut ios = Vec::new();
    for port_info in serialport::available_ports().unwrap() {
        match port_info.port_type {
            serialport::SerialPortType::UsbPort(usb_info) => {
                if usb_info.vid == 0x1209 && usb_info.pid == 0x1776 {
                    println!("Thelio Io at {}", port_info.port_name);

                    let port = serialport::new(port_info.port_name, 115200)
                        .timeout(Duration::from_millis(1))
                        .open()
                        .unwrap();

                    let mut io = Io::new(port, 1000);

                    println!("  reset: {:?}", io.reset());
                    println!("  revision: {:?}", io.revision());
                    println!("  suspend: {:?}", io.suspend());

                    ios.push(io);


                }
            },
            _ => (),
        }
    }

    if ios.is_empty() {
        panic!("failed to find any Thelio Io devices");
    }

    let mut wrapper = Command::new("wrapper/bin/Release/net48/wrapper.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut wrapper_in = wrapper.stdin.take().unwrap();
    let mut wrapper_out = BufReader::new(wrapper.stdout.take().unwrap());

    loop {
        wrapper_in.write_all(b"\n").unwrap();
        let mut line = String::new();
        wrapper_out.read_line(&mut line).unwrap();
        let temp = line.trim().parse::<f64>().unwrap();

        println!("temp: {}", temp);

        for io in ios.iter_mut() {
            for device in &["CPUF", "INTF", "POWB"] {
                println!("device: {}", device);
                println!("  tach: {:?}", io.tach(device));
                println!("  duty: {:?}", io.duty(device));
                println!("  set_duty: {:?}", io.set_duty(device, 0));
                println!("  duty: {:?}", io.duty(device));
            }
        }

        sleep(Duration::new(1, 0));
    }
}
