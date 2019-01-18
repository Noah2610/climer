use std::time::Duration;

use crate::settings::timer;
use crate::ClimerResult;
use super::Timer;
use super::Output;

pub struct TimerBuilder<'a> {
    time:    &'a str,
    format:  Option<&'a str>,
    output:  Option<&'a str>,
    quiet:   bool,
}

impl<'a> TimerBuilder<'a> {
    pub fn new(time: &'a str) -> Self {
        Self {
            time,
            format: None,
            output: None,
            quiet:  false,
        }
    }

    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(&self) -> ClimerResult<Timer> {
        Ok(Timer::new(
                self.time,
                self.format,
                self.build_output()
        )?)
    }

    fn build_output(&self) -> Option<Output> {
        if self.quiet {
            None
        } else {
            Some(if let Some(output) = self.output {
                Output::new(output)
            } else {
                Output::new_with_default()
            })
        }
    }
}
