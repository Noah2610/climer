mod time;
mod builder;
mod output;
mod parser;

use std::time::Duration;

pub use self::builder::TimerBuilder;
pub use self::time::{ Time, TimeBuilder };
use crate::error::ClimerResult;
use self::output::Output;
use self::parser::parse_time;

pub struct Timer {
    time: Time,
}

impl Timer {
    pub fn new(time: &str, format: Option<&str>) -> ClimerResult<Self> {
        Ok(Self {
            time: parse_time(time, format)?
        })
    }
}

