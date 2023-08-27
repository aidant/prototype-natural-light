use chrono::{DateTime, NaiveDateTime, Utc};
use nmea::Nmea;

pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

pub struct Gnss {
    nmea: Nmea,
}

impl Gnss {
    pub fn new() -> Self {
        Self {
            nmea: Nmea::default(),
        }
    }

    fn get_datetime(&self) -> Option<DateTime<Utc>> {
        if let (Some(native_date), Some(native_time)) = (self.nmea.fix_date, self.nmea.fix_time) {
            let native_datetime = NaiveDateTime::new(native_date, native_time);
            let datetime = DateTime::from_utc(native_datetime, Utc);

            Option::Some(datetime)
        } else {
            Option::None
        }
    }

    fn get_coordinates(&self) -> Option<Coordinates> {
        if let (Some(lat), Some(lon)) = (self.nmea.latitude, self.nmea.longitude) {
            Option::Some(Coordinates { lat, lon })
        } else {
            Option::None
        }
    }

    pub fn get(&self) -> (Option<DateTime<Utc>>, Option<Coordinates>) {
        (self.get_datetime(), self.get_coordinates())
    }

    pub fn parse_message(&mut self, message: &str) -> (Option<DateTime<Utc>>, Option<Coordinates>) {
        self.nmea.parse(message);

        self.get()
    }
}
