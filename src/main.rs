#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

extern crate defmt_rtt;
extern crate panic_probe;

use core::ops::Sub;

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::rtc::{Rtc, RtcClockSource, RtcConfig};
use embassy_stm32::{peripherals, Config};
use embassy_time::{Duration, Timer};
use feature_gnss::Coordinates;

use crate::{
    device_adafruit_neopixel_ring::AdafruitNeoPixelRing,
    device_adafruit_ultimate_gps::AdafruitUltimateGps, feature_gnss::Gnss,
    light_characteristics::get_light_characteristics,
};

mod device_adafruit_neopixel_ring;
mod device_adafruit_ultimate_gps;
// mod device_piicodev_oled;
mod feature_gnss;
mod light_characteristics;
mod util_lc_to_rgb;

async fn get_datetime_and_coordinates(
    peri: peripherals::USART1,
    rx_pin: peripherals::PB7,
    rx_dma: peripherals::DMA2_CH2,
) -> (DateTime<Utc>, Coordinates) {
    let mut gnss = Gnss::new();
    let mut adafruit_ultimate_gps = AdafruitUltimateGps::new(peri, rx_pin, rx_dma);

    loop {
        let message = match adafruit_ultimate_gps.read_message().await {
            Result::Ok(message) => {
                info!("{}", message);
                message
            }
            Result::Err(error) => {
                error!("{}", error);
                continue;
            }
        };

        if let (Some(datetime), Some(coordinates)) = gnss.parse_message(&message) {
            return (datetime, coordinates);
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.rtc = Option::Some(RtcClockSource::LSI);

    let p = embassy_stm32::init(config);

    let mut adafruit_neopixel_ring =
        AdafruitNeoPixelRing::new(p.SPI1, p.PA7, p.DMA2_CH3, p.DMA2_CH0);

    adafruit_neopixel_ring.write_off().await;

    let (datetime, coordinates) = get_datetime_and_coordinates(p.USART1, p.PB7, p.DMA2_CH2).await;

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());
    rtc.set_datetime(datetime.naive_utc().into()).unwrap();

    loop {
        let now: DateTime<Utc> = Into::<NaiveDateTime>::into(rtc.now().unwrap()).and_utc();

        let lc = get_light_characteristics(now, &coordinates).unwrap();

        info!("{}", now.timestamp());

        info!(
            "brightness: {} color_temperature: {}",
            lc.brightness, lc.color_temperature
        );

        adafruit_neopixel_ring
            .write_light_characteristics(&lc)
            .await;

        Timer::after(Duration::from_millis(1000)).await;
    }
}
