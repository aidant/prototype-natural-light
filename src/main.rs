#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_rtt_target;

use cortex_m_rt::entry;
use heapless::Vec;
use nmea::Nmea;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::Config,
};
// mod light_characteristics;

use chrono::{TimeZone, Utc as UTC};
// use light_characteristics::get_light_characteristics;

// fn print_light_characteristics() {
//     let datetime = UTC.with_ymd_and_hms(2023, 8, 20, 3, 30, 0).unwrap();

//     rprintln!("{:?}", datetime);

//     let position = light_characteristics::Coordinates {
//         lat: -33.8688,
//         lon: 151.2093,
//     };

//     let light_characteristics = get_light_characteristics(datetime, position).unwrap();

//     rprintln!("{:?}", light_characteristics);
// }

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut nmea = Nmea::default();

    // print_light_characteristics();

    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // let mut delay = cp.SYST.delay(&clocks);

    let gpiob = dp.GPIOB.split();
    let tx1 = gpiob.pb6.into_alternate();
    let rx1 = gpiob.pb7.into_alternate();

    let mut serial = dp
        .USART1
        .serial::<u8>(
            (tx1, rx1),
            Config::default().baudrate(9600_u32.bps()),
            &clocks,
        )
        .unwrap();

    // let gpioc = dp.GPIOC.split();
    // let mut led = gpioc.pc13.into_push_pull_output();

    let mut message = Vec::<u8, 82>::new();

    loop {
        // led.toggle();
        // rprintln!("Hello World!");
        // delay.delay_ms(2000_u32);

        // rprintln!("Hello World!");

        match nb::block!(serial.read()) {
            Ok(byte) => {
                message.push(byte).unwrap();

                if byte == b'\n' {
                    let sentence = core::str::from_utf8(&message).unwrap();
                    rprint!("{}", sentence);

                    nmea.parse(sentence).unwrap();
                    rprintln!("{}", nmea);

                    message.clear();
                }
            }
            Err(error) => rprintln!("unknown error {}", error),
        }
        // ;
    }
}
