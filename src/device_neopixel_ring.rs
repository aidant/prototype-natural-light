use embassy_stm32::{
    dma::NoDma,
    spi::{Config, Spi},
    Peripherals,
};

use crate::light_characteristics::LightCharacteristics;

pub fn set_light_characteristics(p: Peripherals, lc: LightCharacteristics) {
    let mut config = Config::default();

    let spi = Spi::new(p.SPI1, p.PA5, p.PA7, p.PA6, NoDma, NoDma, config);
}
