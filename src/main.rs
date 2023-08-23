#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{dma::NoDma, peripherals};

mod device_adafruit_ultimate_gps;

#[embassy_executor::task]
async fn gps(
    usart: peripherals::USART1,
    pin_rx: peripherals::PB7,
    pin_tx: peripherals::PB6,
    dma_rx: peripherals::DMA2_CH2,
    dma_tx: NoDma,
) {
    device_adafruit_ultimate_gps::get_messages(usart, pin_rx, pin_tx, dma_rx, dma_tx).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hello World!");

    spawner
        .spawn(gps(p.USART1, p.PB7, p.PB6, p.DMA2_CH2, NoDma))
        .unwrap();
}
