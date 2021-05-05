use std::{
    env::current_exe,
    fs,
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

use thelio_io::{
    fan::FanCurve,
    Io,
};

fn main() {
    let smbios = smbioslib::table_load_from_device().unwrap();

    let sys_vendor = smbios.find_map(
        |sys: smbioslib::SMBiosSystemInformation| sys.manufacturer()
    ).unwrap();

    let product_version = smbios.find_map(
        |sys: smbioslib::SMBiosSystemInformation| sys.version()
    ).unwrap();

    let curve = match (sys_vendor.as_str(), product_version.as_str()) {
        ("System76", "thelio-mira-r1") => {
            println!("System76 Thelio Mira (thelio-mira-r1)");
            FanCurve::standard()
        },
        _ => panic!("unsupported sys_vendor '{}' and product_version '{}'", sys_vendor, product_version),
    };

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

    let bin_path = current_exe().unwrap();
    let bin_dir = bin_path.parent().unwrap();

    let ohml_bytes = include_bytes!("../OpenHardwareMonitor/OpenHardwareMonitorLib.dll");
    let ohml_path = bin_dir.join("OpenHardwareMonitorLib.dll");
    fs::write(&ohml_path, &ohml_bytes).unwrap();

    let wrapper_bytes = include_bytes!("../wrapper/bin/Release/net48/wrapper.exe");
    let wrapper_path = bin_dir.join("wrapper.exe");
    fs::write(&wrapper_path, &wrapper_bytes).unwrap();

    let mut wrapper = Command::new(&wrapper_path)
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
        print!("temp: {:02} C", temp as isize);

        if let Some(duty) = curve.get_duty((temp * 100.0) as i16) {
            print!(" duty: {:02}%", (duty as f64 / 100.0) as isize);

            for io in ios.iter_mut() {
                for device in &["CPUF", "INTF"] {
                    io.set_duty(device, duty).unwrap();
                    print!(" {}: {} RPM", device, io.tach(device).unwrap());
                }
            }
        }

        println!();

        sleep(Duration::new(1, 0));
    }
}
