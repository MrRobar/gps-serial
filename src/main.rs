use std::io::{Read, Write};
use std::str::FromStr;
use std::time::Duration;
use std::thread::sleep;
use icu_timezone::{CustomTimeZone, MetazoneCalculator, MetazoneId, TimeZoneBcp47Id};
use nmea_parser::{NmeaParser, ParsedMessage};
use serialport::{SerialPort, SerialPortInfo};
use tzf_rs::{DefaultFinder, deg2num};

struct Parser{
    byte_counter: i32,
    serial_buf: [u8; 1024],
    parser: NmeaParser,
    finder: DefaultFinder,
    connected: bool,
    can_iterate: bool,
    buffer_ordinary: [u8; 512]
}

impl Parser {
    fn form_sentence(&mut self, t: usize){
        let slice = &self.serial_buf[..t];
        for value in slice.iter(){
            let symbol = char::from(*value);

            if *value == b'\r'
            {
                continue;
            }
            if *value == b'\n' {

                let line = String::from_utf8_lossy(&self.buffer_ordinary[0..self.byte_counter as usize]);
                println!("Formed line: {} \n", line);

                self.byte_counter = 0;

                match self.parser.parse_sentence(&line) {
                    Ok(sentence) => {
                        println!("Parsing line...");
                        self.retrieve_data_from_sentence(sentence);
                        return;
                    },
                    Err(error) => {
                        //println!("error parsing sentence: {:?}", error);
                    }
                }
            }
            self.buffer_ordinary[self.byte_counter as usize] = *value;
            self.byte_counter += 1;
        }
    }

    fn retrieve_data_from_sentence(&mut self, sentence: ParsedMessage){
        //println!("{:?}", sentence);
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
                let timezone = self.finder.get_tz_name(rmc.longitude.unwrap(), rmc.latitude.unwrap());
                println!("Time: {}", timezone);
                if let Some(timestamp) = rmc.timestamp {
                    println!("{:?} \n", timestamp);
                }
            },
            _ => {}
        }
    }
}

fn main() {
    let mut parser_struct = Parser{
        byte_counter: 0,
        serial_buf: [0u8; 1024],
        parser: NmeaParser::new(),
        finder: DefaultFinder::new(),
        connected: false,
        can_iterate: true,
        buffer_ordinary: [0u8; 512],
    };

    let ports = serialport::available_ports().expect("System error");
    let port = ports.first().expect("No ports available");
    println!("Receiving data on {} at {} baud:", &port.port_name, 9600);
    let mut port= serialport::new(&port.port_name, 9600)
    .timeout(Duration::from_millis(10))
        .open()
        .expect(&format!("Unable to open serial port '{}'", port.port_name));

    let mut h = 0;
    let mut m = 0;
    let mut s = 0;

    let delay = Duration::from_secs(1);

    loop {
        parser_struct.byte_counter = 0;
        match port.read(parser_struct.serial_buf.as_mut_slice()) {
            Ok(t) => {
                parser_struct.form_sentence(t);
                sleep(delay)
                // std::io::stdout().write_all(&serial_buf[..t]).unwrap()
            },

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}

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