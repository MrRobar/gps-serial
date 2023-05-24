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

struct Location {
    latitude: [u8; 10],
    longitude: [u8; 10],
}

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
    let mut serial = dp.USART1.serial((tx_pin, rx_pin), Config::default().baudrate(9600.bps()), &clocks).unwrap();
    let (mut tx, mut rx): (stm32f4xx_hal::serial::Tx<stm32f4xx_hal::pac::USART1, u8>, stm32f4xx_hal::serial::Rx<stm32f4xx_hal::pac::USART1, u8>) = serial.split();
    let mut value: u8 = 0;
    let mut location = Location {
        latitude: [0; 10],
        longitude: [0; 10],
    }; // структура для хранения координат
    let mut rmc_msg = [0u8; 82]; // Массив байтов для хранения RMC сообщения
    let mut rmc_len = 0; // Длина RMC сообщения
    let mut time_data = [0u8; 20]; // Массив байтов для хранения временных данных
    let mut time_len = 0; // Длина временных данных

    loop {
        match block!(rx.read()) {
            Ok(byte) => {
                if byte == b'$' {
                    rmc_len = 0;
                }
                let mut symbol = char::from(byte);
                //defmt::info!("{}", symbol);

                rmc_msg[rmc_len] = byte;
                rmc_len += 1;

                if rmc_len >= 6 && &rmc_msg[3..6] == b"RMC" {
                    // Обработка RMC сообщения
                    let mut comma_count = 0;
                    defmt::info!("Working on message...");
                    for i in 0..rmc_len {

                        if rmc_msg[i] == b',' {
                            comma_count += 1;
                            if comma_count == 1 {
                                // Начало нужной информации
                                time_len = 0;
                            } else if comma_count == 2 {
                                // Конец нужной информации
                                break;
                            }
                        } else if comma_count == 1 {
                            // Записываем информацию в массив
                            time_data[time_len] = rmc_msg[i];
                            time_len += 1;
                        }
                    }
                    //обработка временных данных
                    // let mut hour = [0u8; 2];
                    // hour[0] = time_data[0] / 10 + b'0';
                    // hour[1] = time_data[0] % 10 + b'0';
                    //
                    // let mut minute = [0u8; 2];
                    // minute[0] = time_data[1] / 10 + b'0';
                    // minute[1] = time_data[1] % 10 + b'0';
                    //
                    // let mut second = [0u8; 2];
                    // second[0] = time_data[2] / 10 + b'0';
                    // second[1] = time_data[2] % 10 + b'0';
                    // let mut h = char::from(hour[0]);
                    // let mut h1 = char::from(hour[1]);
                    // let mut m = char::from(hour[0]);
                    // let mut m1 = char::from(hour[1]);
                    // let mut s = char::from(hour[0]);
                    // let mut s1 = char::from(hour[1]);
                    //defmt::info!("Time: {}{}:{}{}:{}{}", h, h1, m, m1, s, s1);
                    //panic!("Took results, finishing program");

                }
            }
            Err(_) => {
                // Обработка ошибки при чтении данных
                // ...
                //defmt::info!("Error occured...");
            }
        }
    }
}