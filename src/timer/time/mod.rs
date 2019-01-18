mod builder;

pub use self::builder::TimeBuilder;

#[derive(Debug, PartialEq)]
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
}
