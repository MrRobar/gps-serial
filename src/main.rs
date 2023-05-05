#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::singleton;
use cortex_m_rt::entry;

use stm32f4xx_hal::{pac, prelude::*};
use stm32f4xx_hal::adc::{Adc, SampleTime};
use stm32f4xx_hal::serial::{Config, Serial};

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::time::Duration;
use nmea_parser::{chrono, NmeaParser, ParsedMessage};
use tzf_rs::{DefaultFinder, deg2num};
use chrono::{DateTime, Utc, Timelike};

struct Parser{
    position_index: i32,
    buffer: [u8; 1024],
    parser: NmeaParser,
    finder: DefaultFinder,
}

impl  Parser {
    fn form_sentence(&mut self, slice: &[u8]){
        for b in slice { // На контроллере будет просто чтение по одному байту
            if *b != b'\n' {
                if *b == b'\r' {
                    let end = self.position_index;
                    self.position_index = 0;
                    let line = core::str::from_utf8(&self.buffer[0..end as usize]).unwrap(); // ERROR OCCURS HERE. SOMETIMES RESULT IS NULL
                    if line.starts_with('$') {
                        //println!("Line:    {:?}", line);
                        if let Ok(sentence) = self.parser.parse_sentence(line) {
                            match sentence {
                                ParsedMessage::Rmc(rmc) => {
                                    if let Some(lon) = rmc.longitude {
                                        if let Some(lat) = rmc.latitude {
                                            println!("RMC pos: {} {}", lon, lat);
                                            let timezone = self.finder.get_tz_name(rmc.longitude.unwrap(), rmc.latitude.unwrap());
                                            println!("Time: {}", timezone);
                                            self.get_time_offset(timezone);
                                            if let Some(timestamp) = rmc.timestamp {
                                                println!("{:?} \n", timestamp);
                                                let hour = timestamp.hour();
                                                let minute = timestamp.minute();
                                                let second = timestamp.second();
                                                println!("Hours: {}. Minutes: {}. Seconds: {}", hour, minute, second);
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
                    self.buffer[self.position_index as usize] = *b;
                    self.position_index += 1;
                }
            }
        }
    }

    fn get_time_offset(self, timezone: &str){
        let file = File::open("timezones.txt").expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut i = 0;
        for line in reader.lines(){
            let record = line.expect("Failed to read line");
            let fields : Vec<&str> = record.split(',').collect();
            if fields[0] == timezone {
                println!("{}", fields[1]);
                break;
            }
        }
    }

    fn parse_line(&mut self, line: &str){

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

    let mut serial_buf = [0u8; 1024];

    let mut parser_struct = Parser{
        position_index: 0,
        buffer: [0u8; 1024],
        parser: NmeaParser::new(),
        finder: DefaultFinder::new(),
    };

    println!("{:?}", parser_struct.finder.timezonenames());

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                //let slice = &serial_buf[..t];
                //parser_struct.form_sentence(slice);
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}

// use stm32_usart::{Usart, Config};
// use cortex_m_rt::entry;
//
// #[entry]
// fn main() -> ! {
//     let dp = stm32::Peripherals::take().unwrap();
//     let gpioa = dp.GPIOA.split();
//     let tx_pin = gpioa.pa2.into_alternate_af7();
//     let rx_pin = gpioa.pa3.into_alternate_af7();
//
//     let config = Config::default().baudrate(115200.bps());
//     let mut usart = Usart::new(dp.USART2, (tx_pin, rx_pin), config);
//
//     loop {
//         usart.write(b'A').unwrap();
//     }
// }