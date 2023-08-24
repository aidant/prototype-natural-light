use crate::light_characteristics::LightCharacteristics;
use chrono::{DateTime, Utc};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};

pub fn set_datetime_and_light_characteristics(datetime: DateTime<Utc>, lc: LightCharacteristics) {
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::GREEN);

    Text::new("Hello,\nRust!", Point::new(2, 28), style).draw(&mut display)?;
}
