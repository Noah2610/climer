pub mod prelude {
    pub use super::Time;
    pub use super::TimeBuilder;
}

pub mod parser;
mod builder;
mod time_conversion;

use std::fmt;

pub use self::builder::TimeBuilder;
pub use self::time_conversion::TimeConversion;

#[derive(Debug, PartialEq, TimeConversion)]
pub struct Time {
    hours:        u32,
    minutes:      u32,
    seconds:      u32,
    milliseconds: u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32, seconds: u32, milliseconds: u32) -> Self {
        Self { hours, minutes, seconds, milliseconds }
    }

    pub fn as_hours(&self) -> f32 {
        self.hours as f32 +
            self.minutes as f32 / 60.0 +
            self.seconds as f32 / 60.0 / 60.0 +
            self.milliseconds as f32 / 1000.0 / 60.0 / 60.0
    }

    pub fn as_minutes(&self) -> f32 {
        self.hours as f32 * 60.0 +
            self.minutes as f32 +
            self.seconds as f32 / 60.0 +
            self.milliseconds as f32 / 1000.0 / 60.0
    }

    pub fn as_seconds(&self) -> f32 {
        self.hours as f32 * 60.0 * 60.0 +
            self.minutes as f32 * 60.0 +
            self.seconds as f32 +
            self.milliseconds as f32 / 1000.0
    }

    pub fn as_milliseconds(&self) -> f32 {
        self.hours as f32 * 60.0 * 60.0 * 1000.0 +
            self.minutes as f32 * 60.0 * 1000.0 +
            self.seconds as f32 * 1000.0 +
            self.milliseconds as f32
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:02}:{:02}:{:02}.{:04}", self.hours, self.minutes, self.seconds, self.milliseconds)
    }
}

#[cfg(test)]
mod tests;
