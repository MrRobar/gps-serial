#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::register::msp::read;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::singleton;
use cortex_m_rt::entry;
use defmt::export::char;

use stm32f4xx_hal as hal;
use stm32f4xx_hal::block;
use stm32f4xx_hal::serial::Config;
use crate::hal::{pac, prelude::*};

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
    let mut rmc_msg = [0u8; 82]; // Массив байтов для хранения RMC сообщения
    let mut rmc_len = 0; // Длина RMC сообщения
    let mut time_data: [u8; 9] = [b'0'; 9];
    let mut index = 0;
    let mut kaliningrad_offset = 2;
    let mut res_h = 0;
    let mut m = 0;
    let mut s = 0;

    loop {
        match block!(rx.read()) {
            Ok(byte) => {
                if byte == b'$' {
                    rmc_len = 0;
                }
                if byte == b'\n'{
                    if &rmc_msg[3..6] == b"RMC" {
                        for (i, &byte) in rmc_msg.iter().enumerate() {
                            if byte == b',' && index == 0 {
                                index = i+1;
                            } else if byte == b',' && index != 0 {
                                let time_slice = &rmc_msg[index..i];
                                for (j, &t) in time_slice.iter().enumerate() {
                                    if j >= 6 {
                                        break;
                                    }
                                    time_data[j] = t;
                                }
                                break;
                            }
                        }
                        let h = ((time_data[0] - 48) * 10 + (time_data[1] - 48)) as u8; // первые два байта - часы
                        res_h = h + kaliningrad_offset;
                        m = ((time_data[2] - 48) * 10 + (time_data[3] - 48)) as u8; // следующие два байта - минуты
                        s = ((time_data[4] - 48) * 10 + (time_data[5] - 48)) as u8; // последние два байта - секунды
                        defmt::info!("Current time: {}:{}:{}", res_h, m, s);
                        rmc_len = 0;
                        index = 0;
                    }
                }

                rmc_msg[rmc_len] = byte;
                rmc_len += 1;
            }
            Err(_) => {
                // Обработка ошибки при чтении данных
            }
        }
        tx.bwrite_all(&[res_h, m, s]).unwrap();
        //tx.bflush();
    }
}