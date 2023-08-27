struct LibMFloatOptions;

use libm::{acos, asin, atan, atan2, cos, sin, tan, trunc};

use crate::feature_gnss::Coordinates;

impl spa::FloatOps for LibMFloatOptions {
    fn sin(x: f64) -> f64 {
        sin(x)
    }
    fn cos(x: f64) -> f64 {
        cos(x)
    }
    fn tan(x: f64) -> f64 {
        tan(x)
    }
    fn asin(x: f64) -> f64 {
        asin(x)
    }
    fn acos(x: f64) -> f64 {
        acos(x)
    }
    fn atan(x: f64) -> f64 {
        atan(x)
    }
    fn atan2(y: f64, x: f64) -> f64 {
        atan2(y, x)
    }
    fn trunc(x: f64) -> f64 {
        trunc(x)
    }
}

#[derive(Debug)]
struct SolarPosition {
    azimuth: f64,
    elevation: f64,
}

fn get_solar_position(
    datetime: chrono::DateTime<chrono::Utc>,
    coordinates: &Coordinates,
) -> Option<SolarPosition> {
    match spa::solar_position::<LibMFloatOptions>(datetime, coordinates.lat, coordinates.lon) {
        Result::Err(error) => match error {
            spa::SpaError::BadParam => Option::None,
        },
        Result::Ok(position) => Option::Some(SolarPosition {
            azimuth: position.azimuth,
            elevation: 90.0 - position.zenith_angle,
        }),
    }
}

fn get_brightness(position: &SolarPosition) -> f64 {
    sin(position.elevation.to_radians()).max(0.0)
}

fn get_color_temperature(position: &SolarPosition) -> f64 {
    3500.0 + (2000.0 * position.elevation / 90.0)
}

#[derive(Debug)]
pub struct LightCharacteristics {
    pub brightness: f64,
    pub color_temperature: f64,
}

pub fn get_light_characteristics(
    datetime: chrono::DateTime<chrono::Utc>,
    coordinates: &Coordinates,
) -> Option<LightCharacteristics> {
    let solar_position = get_solar_position(datetime, &coordinates)?;

    Option::Some(LightCharacteristics {
        brightness: get_brightness(&solar_position),
        color_temperature: get_color_temperature(&solar_position),
    })
}
