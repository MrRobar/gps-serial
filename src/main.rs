use std::io::{Read, Write};
use std::str::FromStr;
use std::time::Duration;
use icu_timezone::{CustomTimeZone, MetazoneCalculator, MetazoneId, TimeZoneBcp47Id};
use nmea_parser::{NmeaParser, ParsedMessage};
use tzf_rs::{DefaultFinder, deg2num};

fn main() {
    let ports = serialport::available_ports().expect("System error");
    let port = ports.first().expect("No ports available");
    println!("Receiving data on {} at {} baud:", &port.port_name, 9600);

    let mut port = serialport::new(&port.port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect(&format!("Unable to open serial port '{}'", port.port_name));

    //let mut buffer = Vec::new();
    let mut last = 0;
    let mut serial_buf = [0u8; 1024];

    let mut parser = NmeaParser::new();

    let finder = DefaultFinder::new();

    let mut connected = false;

    let mut bufferOrdinary = [0u8; 512];

    let mut h = 0;
    let mut m = 0;
    let mut s = 0;

    //Високосные года!!

    // for (i, b) in buffer.iter().enumerate() {
    //     match i {
    //         0 => h = 10 * (*b - b'0'), //ASCII TO NUMBER HACK
    //         1 => h += (*b - b'0'),
    //         2 => m = 10 * (*b - b'0'),
    //         3 => m += (*b - b'0'), //..
    //         4 => s += 10 * (*b - b'0'),
    //         5 => s += (*b - b'0'),
    //         _ => {}
    //     }
    // }

    //USART по пину (TX, RX на картинке)
    //https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/rtic-usart-shell.rs

    loop {
        let mut  i = 0;
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {

                let slice = &serial_buf[..t];
                let lenOrdinary = bufferOrdinary.len();
                for value in slice.iter(){
                    let symbol = char::from(*value);
                    println!("{}", symbol);
                    if *value == b'\r'{continue;}
                    if *value == b'\n' {
                        let line = String::from_utf8_lossy(&bufferOrdinary[0..i]);
                        println!("Forming line from {i} symbols");
                        println!("Formed line: {line}");
                        i = 0;
                        if let Ok(my_sentence) = parser.parse_sentence(line.as_ref()){
                            println!("myLine was parsed successfully");
                        }
                        if let Ok(sentence) = parser.parse_sentence(line.as_ref()) {
                            println!("{:?}", sentence);
                            match sentence {
                                ParsedMessage::Gll(gll) => {
                                    println!("Navigation: {:?}", gll);
                                }
                                ParsedMessage::Rmc(rmc) => {
                                    if let Some(lon) = rmc.longitude {
                                        if let Some(lat) = rmc.latitude {
                                            println!("RMC pos: {} {}", lon, lat);
                                        }
                                    }
                                    // let timezone = finder.get_tz_name(rmc.longitude.unwrap(), rmc.latitude.unwrap());
                                    // println!("Time:    {}", timezone);
                                    /*if let Some(timestamp) = rmc.timestamp {
                                    }*/
                                },
                                _ => {}
                            }
                        }
                        return;
                    }
                    bufferOrdinary[i] = *value;
                    i += 1;
                }
                // std::io::stdout().write_all(&serial_buf[..t]).unwrap()
            },

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}