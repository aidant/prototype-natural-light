use crate::light_characteristics::LightCharacteristics;

use libm::powf;

pub fn lc_to_rgb(lc: &LightCharacteristics) -> (u8, u8, u8) {
    let mut red: f32;
    let mut green: f32;
    let mut blue: f32;

    let kelvin = lc.color_temperature as f32;
    let brightness = lc.brightness as f32;

    if kelvin < 6600.0 {
        red = powf(kelvin / 100.0, -0.1332047592);
        green = powf(kelvin / 100.0, -0.0755148492);
        blue = powf(6600.0 / kelvin, 0.64373109);
    } else {
        red = powf(kelvin / 100.0 - 60.0, -0.0755148492);
        green = powf(kelvin / 100.0 - 60.0, -0.0913738780);
        blue = 1.0;
    }

    // Adjust for brightness
    red *= brightness;
    green *= brightness;
    blue *= brightness;

    red = (red * 255.0).clamp(0.0, 255.0);
    green = (green * 255.0).clamp(0.0, 255.0);
    blue = (blue * 255.0).clamp(0.0, 255.0);

    (red as u8, green as u8, blue as u8)
}
