#![no_std]
#![no_main]

use cortex_m::register::msp::read;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::singleton;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use stm32f4xx_hal::block;
use stm32f4xx_hal::serial::Config;
use crate::hal::{pac, prelude::*};
// use stm32f4xx_hal::{pac, prelude::*};
// use stm32f4xx_hal::adc::{Adc, SampleTime};
// use stm32f4xx_hal::serial::{Config, Serial};

// use std::fs::File;
// use std::io::{BufRead, BufReader, Read, Write};
// use std::time::Duration;
// use nmea_parser::{chrono, NmeaParser, ParsedMessage};
// use tzf_rs::{DefaultFinder, deg2num};
// use chrono::{DateTime, Utc, Timelike};


// impl  Parser {
//     fn form_sentence(&mut self, slice: &[u8]){
//         for b in slice { // На контроллере будет просто чтение по одному байту
//             if *b != b'\n' {
//                 if *b == b'\r' {
//                     let end = self.position_index;
//                     self.position_index = 0;
//                     let line = core::str::from_utf8(&self.buffer[0..end as usize]).unwrap(); // ERROR OCCURS HERE. SOMETIMES RESULT IS NULL
//                     if line.starts_with('$') {
//                         //println!("Line:    {:?}", line);
//                         if let Ok(sentence) = self.parser.parse_sentence(line) {
//                             match sentence {
//                                 ParsedMessage::Rmc(rmc) => {
//                                     if let Some(lon) = rmc.longitude {
//                                         if let Some(lat) = rmc.latitude {
//                                             println!("RMC pos: {} {}", lon, lat);
//                                             let timezone = self.finder.get_tz_name(rmc.longitude.unwrap(), rmc.latitude.unwrap());
//                                             println!("Time: {}", timezone);
//                                             self.get_time_offset(timezone);
//                                             if let Some(timestamp) = rmc.timestamp {
//                                                 println!("{:?} \n", timestamp);
//                                                 let hour = timestamp.hour();
//                                                 let minute = timestamp.minute();
//                                                 let second = timestamp.second();
//                                                 println!("Hours: {}. Minutes: {}. Seconds: {}", hour, minute, second);
//                                             }
//                                         }
//                                     }
//                                 },
//                                 _ => {}
//                             }
//                         }
//                     } else {
//                         eprintln!("Broken:  {:?}", line);
//                     }
//                 } else {
//                     self.buffer[self.position_index as usize] = *b;
//                     self.position_index += 1;
//                 }
//             }
//         }
//     }
//
//     fn get_time_offset(self, timezone: &str){
//         let file = File::open("timezones.txt").expect("Failed to open file");
//         let reader = BufReader::new(file);
//
//         let mut i = 0;
//         for line in reader.lines(){
//             let record = line.expect("Failed to read line");
//             let fields : Vec<&str> = record.split(',').collect();
//             if fields[0] == timezone {
//                 println!("{}", fields[1]);
//                 break;
//             }
//         }
//     }
//
//     fn parse_line(&mut self, line: &str){
//
//     }
// }

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();
    let mut delay = dp.TIM1.delay_ms(&clocks);

    //define RX/TX pins
    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();

    //configure serial
    let mut serial = dp.USART1.serial((tx_pin, rx_pin), Config::default().baudrate(9600.bps()).wordlength_9(), &clocks).unwrap();
    let (mut tx, mut rx): (stm32f4xx_hal::serial::Tx<stm32f4xx_hal::pac::USART1, u8>, stm32f4xx_hal::serial::Rx<stm32f4xx_hal::pac::USART1, u8>) = serial.split();
    let mut value: u8 = 0;

    loop {
        match block!(rx.read()) {
            Ok(byte) => {
                // Обработка полученного байта от GPS датчика
                // ...
                defmt::info!("{}", byte);
            }
            Err(_) => {
                // Обработка ошибки при чтении данных
                // ...
                //defmt::info!("Error occured...");
            }
        }
    }
    // let ports = serialport::available_ports().expect("System error");
    // let port = ports.first().expect("No ports available");
    // defmt::info!("Receiving data on {} at {} baud:", &port.port_name, 9600);
    //
    // let mut port = serialport::new(&port.port_name, 9600)
    //     .timeout(Duration::from_millis(10))
    //     .open()
    //     .expect(&format!("Unable to open serial port '{}'", port.port_name));
    //
    // let mut serial_buf = [0u8; 1024];
    //
    // // let mut parser_struct = Parser{
    // //     position_index: 0,
    // //     buffer: [0u8; 1024],
    // //     parser: NmeaParser::new(),
    // //     finder: DefaultFinder::new(),
    // // };
    // defmt::info!("Starting reading from port...");
    // loop {
    //     match port.read(serial_buf.as_mut_slice()) {
    //         Ok(t) => {
    //             //let slice = &serial_buf[..t];
    //             //parser_struct.form_sentence(slice);
    //         },
    //         Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
    //         other => {
    //             other.unwrap();
    //         },
    //     }
    // }
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