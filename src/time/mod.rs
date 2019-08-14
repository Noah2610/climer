pub mod prelude {
    pub use super::Time;
    pub use super::TimeBuilder;
}

mod builder;
pub mod parser;
mod time_conversion;

use std::cmp::{self, Ordering};
use std::fmt;
use std::ops;
use std::time::Duration;

pub use self::builder::TimeBuilder;
pub use self::time_conversion::TimeConversion;

#[derive(Debug, Clone, Copy, PartialEq, TimeConversion)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Time {
    hours:        u64,
    minutes:      u64,
    seconds:      u64,
    milliseconds: u64,
    nanoseconds:  u64,
}

impl Time {
    pub fn new(
        hours: u64,
        minutes: u64,
        seconds: u64,
        milliseconds: u64,
        nanoseconds: u64,
    ) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
            nanoseconds,
        }
    }

    pub fn zero() -> Self {
        Self {
            hours:        0,
            minutes:      0,
            seconds:      0,
            milliseconds: 0,
            nanoseconds:  0,
        }
    }

    pub fn as_hours(&self) -> f32 {
        self.hours as f32
            + self.minutes as f32 / 60.0
            + self.seconds as f32 / 60.0 / 60.0
            + self.milliseconds as f32 / 1000.0 / 60.0 / 60.0
            + self.nanoseconds as f32 / 1_000_000.0 / 1000.0 / 60.0 / 60.0
    }

    pub fn as_minutes(&self) -> f32 {
        self.hours as f32 * 60.0
            + self.minutes as f32
            + self.seconds as f32 / 60.0
            + self.milliseconds as f32 / 1000.0 / 60.0
            + self.nanoseconds as f32 / 1_000_000.0 / 1000.0 / 60.0
    }

    pub fn as_seconds(&self) -> f32 {
        self.hours as f32 * 60.0 * 60.0
            + self.minutes as f32 * 60.0
            + self.seconds as f32
            + self.milliseconds as f32 / 1000.0
            + self.nanoseconds as f32 / 1_000_000.0 / 1000.0
    }

    pub fn as_milliseconds(&self) -> f32 {
        self.hours as f32 * 60.0 * 60.0 * 1000.0
            + self.minutes as f32 * 60.0 * 1000.0
            + self.seconds as f32 * 1000.0
            + self.milliseconds as f32
            + self.nanoseconds as f32 / 1_000_000.0
    }

    pub fn as_nanoseconds(&self) -> f32 {
        self.hours as f32 * 60.0 * 60.0 * 1000.0 * 1_000_000.0
            + self.minutes as f32 * 60.0 * 1000.0 * 1_000_000.0
            + self.seconds as f32 * 1000.0 * 1_000_000.0
            + self.milliseconds as f32 * 1_000_000.0
            + self.nanoseconds as f32
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        let new = TimeBuilder::new()
            .seconds(duration.as_secs() as u64)
            .nanoseconds(duration.subsec_nanos() as u64)
            .build();
        new
    }
}

impl ops::Add for Time {
    type Output = Time;
    fn add(self, other: Time) -> Self {
        TimeBuilder::new()
            .hours(self.h() + other.h())
            .minutes(self.m() + other.m())
            .seconds(self.s() + other.s())
            .milliseconds(self.ms() + other.ms())
            .nanoseconds(self.ns() + other.ns())
            .build()
    }
}

impl ops::AddAssign for Time {
    fn add_assign(&mut self, other: Time) {
        self.add_hours(other.h());
        self.add_minutes(other.m());
        self.add_seconds(other.s());
        self.add_milliseconds(other.ms());
        self.add_nanoseconds(other.ns());
    }
}

impl ops::Sub for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Self {
        let mut nanoseconds =
            self.as_nanoseconds() as u64 - other.as_nanoseconds() as u64;
        let mut milliseconds = nanoseconds / 1_000_000;
        nanoseconds -= milliseconds * 1_000_000;
        let mut seconds = milliseconds / 1000;
        milliseconds -= seconds * 1000;
        let mut minutes = seconds / 60;
        seconds -= minutes * 60;
        let hours = minutes / 60;
        minutes -= hours * 60;
        TimeBuilder::new()
            .hours(hours)
            .minutes(minutes)
            .seconds(seconds)
            .milliseconds(milliseconds)
            .nanoseconds(nanoseconds)
            .build()
    }
}

// TODO
// impl ops::SubAssign for Time {
//     fn sub_assign(&mut self, other: Time) {
//     }
// }

impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        self.as_hours().partial_cmp(&other.as_hours())
    }
}

#[cfg(test)]
mod tests;
