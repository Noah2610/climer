use std::time::Duration;

use crate::settings::timer;
use crate::ClimerResult;
use super::Timer;

pub struct TimerBuilder<'a> {
    time:    &'a str,
    format:  Option<&'a str>,
}

impl<'a> TimerBuilder<'a> {
    pub fn new(time: &'a str) -> Self {
        Self {
            time,
            format: None,
        }
    }

    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> ClimerResult<Timer> {
        Ok(Timer::new(
                self.time,
                self.format,
        )?)
    }
}
