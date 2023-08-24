#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Peripherals;

mod device_adafruit_ultimate_gps;

#[embassy_executor::task]
async fn gps(p: Peripherals) {
    device_adafruit_ultimate_gps::get_messages(p).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hello World!");

    spawner.spawn(gps(p)).unwrap();
}
