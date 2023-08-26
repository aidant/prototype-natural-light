#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use defmt::*;
use embassy_executor::Spawner;

use crate::{
    device_adafruit_neopixel_ring::AdafruitNeoPixelRing,
    device_adafruit_ultimate_gps::AdafruitUltimateGps,
};

mod device_adafruit_neopixel_ring;
mod device_adafruit_ultimate_gps;
// mod device_piicodev_oled;
mod light_characteristics;
mod util_lc_to_rgb;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hello World!");

    let mut adafruit_ultimate_gps = AdafruitUltimateGps::new(p.USART1, p.PB7, p.DMA2_CH2);
    let mut adafruit_neopixel_ring =
        AdafruitNeoPixelRing::new(p.SPI1, p.PA7, p.DMA2_CH3, p.DMA2_CH0);

    adafruit_neopixel_ring
        .write_light_characteristics(&light_characteristics::LightCharacteristics {
            brightness: 0.5,
            color_temperature: 2000.0,
        })
        .await;

    loop {
        let message = adafruit_ultimate_gps.read_message().await;

        info!("{}", message.strip_suffix("\r\n").unwrap());
    }

    // spawner.spawn(gps(p)).unwrap();
}
