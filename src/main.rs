use std::io::{Read, Write};
use std::time::Duration;
use nmea_parser::{NmeaParser, ParsedMessage};
use tzf_rs::{DefaultFinder, deg2num};

struct Parser{
    position_index: i32,
    serial_buf: [u8; 1024],
    parser: NmeaParser,
    finder: DefaultFinder,
}

impl  Parser {
    fn form_sentence(){
        
    }
}

fn main() {
    let ports = serialport::available_ports().expect("System error");
    let port = ports.first().expect("No ports available");
    println!("Receiving data on {} at {} baud:", &port.port_name, 9600);

    let mut port = serialport::new(&port.port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect(&format!("Unable to open serial port '{}'", port.port_name));

    let mut buffer = [0u8; 1024];
    let mut pos = 0;

    let mut parser = NmeaParser::new();
    let mut finder = DefaultFinder::new();

    let mut serial_buf = [0u8; 1024];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let slice = &serial_buf[..t];

                for b in slice { // На контроллере будет просто чтение по одному байту
                    if *b != b'\n' {
                        if *b == b'\r' {
                            let end = pos;
                            pos = 0;
                            let line = core::str::from_utf8(&buffer[0..end]).unwrap();
                            if line.starts_with('$') {
                                // println!("Line:    {:?}", line);
                                if let Ok(sentence) = parser.parse_sentence(line) {
                                    //println!("Line:    {:?}", sentence);
                                    match sentence {
                                        ParsedMessage::Rmc(rmc) => {
                                            if let Some(lon) = rmc.longitude {
                                                if let Some(lat) = rmc.latitude {
                                                    println!("RMC pos: {} {}", lon, lat);
                                                    let timezone = finder.get_tz_name(rmc.longitude.unwrap(), rmc.latitude.unwrap());
                                                    println!("Time: {}", timezone);
                                                    if let Some(timestamp) = rmc.timestamp {
                                                        println!("{:?} \n", timestamp);
                                                    }
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            } else {
                                eprintln!("Broken:  {:?}", line);
                            }
                        } else {
                            buffer[pos] = *b;
                            pos += 1;
                        }
                    }
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}
