use std::{
    str,
    thread::sleep,
    time::Duration,
};

pub mod fan;

pub struct Io {
    port: Box<dyn serialport::SerialPort>,
    timeout: u32,
}

impl Io {
    pub fn new(port: Box<dyn serialport::SerialPort>, timeout: u32) -> Self {
        Self { port, timeout }
    }

    pub fn command(&mut self, command: &str) -> Result<Vec<String>, String> {
        // println!("> {}", command);
        self.port.write(format!("{}\r", command).as_bytes()).unwrap();

        let mut responses = Vec::new();
        let mut buf = [0; 4096];
        for _ in 0..self.timeout {
            let to_read = self.port.bytes_to_read().unwrap();
            if to_read > 0 {
                let count = self.port.read(&mut buf[..to_read as usize]).unwrap();
                if count > 0 {
                    let string = str::from_utf8(&buf[..count]).unwrap();

                    for line in string.lines() {
                        // println!("< {}", line);
                        match line.trim() {
                            "" => (),
                            "ERROR" => return Err(format!("Io command '{}' returned ERROR", command)),
                            "OK" => return Ok(responses),
                            response => responses.push(response.to_string()),
                        }
                    }
                }
            }
            sleep(Duration::from_millis(1))
        }

        Err(format!("Io command '{}' timed out", command))
    }

    pub fn command_string(&mut self, command: &str) -> Result<String, String> {
        let mut responses = self.command(command)?;
        if responses.len() == 1 {
            Ok(responses.remove(0))
        } else {
            Err(format!("Io command '{}' returned {} responses instead of 1", command, responses.len()))
        }
    }

    pub fn command_u16(&mut self, command: &str) -> Result<u16, String> {
        let response = self.command_string(command)?;
        match u16::from_str_radix(&response, 16) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("Io command '{}' failed to parse response '{}': {}", command, response, err)),
        }
    }

    pub fn reset(&mut self) -> Result<(), String> {
        self.command("IoRSET")?;
        Ok(())
    }

    pub fn tach(&mut self, device: &str) -> Result<u16, String> {
        if device.len() != 4 {
            return Err(format!("Io tach device length was {} instead of 4", device.len()));
        }

        self.command_u16(&format!("IoTACH{}", device))
    }

    pub fn duty(&mut self, device: &str) -> Result<u16, String> {
        if device.len() != 4 {
            return Err(format!("Io duty device length was {} instead of 4", device.len()));
        }

        self.command_u16(&format!("IoDUTY{}", device))
    }

    pub fn set_duty(&mut self, device: &str, value: u16) -> Result<(), String> {
        if device.len() != 4 {
            return Err(format!("Io set_duty device length was {} instead of 4", device.len()));
        }

        self.command(&format!("IoDUTY{}{:04X}", device, value))?;
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<u16, String> {
        self.command_u16("IoSUSP")
    }

    pub fn set_suspend(&mut self, value: u16) -> Result<(), String> {
        self.command(&format!("IoSUSP{:04X}", value))?;
        Ok(())
    }

    pub fn revision(&mut self) -> Result<String, String> {
        self.command_string("IoREVISION")
    }
}
