use embassy_stm32::{
    peripherals,
    spi::{Config, Spi},
    time::Hertz,
};
use smart_leds::{gamma, RGB8};
use ws2812_async::Ws2812;

use crate::{light_characteristics::LightCharacteristics, util_lc_to_rgb::lc_to_rgb};

pub struct AdafruitNeoPixelRing<'a> {
    driver: Ws2812<
        Spi<'a, peripherals::SPI1, peripherals::DMA2_CH3, peripherals::DMA2_CH0>,
        { 12 * 12 },
    >,
}

impl AdafruitNeoPixelRing<'_> {
    pub fn new(
        peri: peripherals::SPI1,
        mosi: peripherals::PA7,
        tx_dma: peripherals::DMA2_CH3,
        rx_dma: peripherals::DMA2_CH0,
    ) -> Self {
        let mut config = Config::default();
        config.frequency = Hertz(4_000_000);

        let spi = Spi::new_txonly_nosck(peri, mosi, tx_dma, rx_dma, config);

        let ws = Ws2812::<_, { 12 * 12 }>::new(spi);

        Self { driver: ws }
    }

    pub async fn write_off(&mut self) {
        let mut data = [RGB8::default(); 12];

        for i in 0..12 {
            data[i] = RGB8 { r: 0, g: 0, b: 0 };
        }

        self.driver
            .write(gamma(data.iter().cloned()))
            .await
            .unwrap();
    }

    pub async fn write_light_characteristics(&mut self, lc: &LightCharacteristics) {
        let mut data = [RGB8::default(); 12];

        for i in 0..12 {
            let (r, g, b) = lc_to_rgb(&lc);
            data[i] = RGB8 { r, g, b };
        }

        self.driver
            .write(gamma(data.iter().cloned()))
            .await
            .unwrap();
    }
}
