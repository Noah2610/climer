use std::time::Duration;

use crate::settings::timer;
use crate::ClimerResult;
use crate::time::Time;
use super::Timer;
use super::Output;

pub struct TimerBuilder<'a> {
    time:           &'a str,
    quiet:          bool,
    format:         Option<&'a str>,
    output:         Option<&'a str>,
    print_interval: Option<Time>,
    write:          Option<&'a str>,
}

impl<'a> TimerBuilder<'a> {
    pub fn new(time: &'a str) -> Self {
        Self {
            time,
            quiet:          false,
            format:         None,
            output:         None,
            print_interval: None,
            write:          None,
        }
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }

    pub fn print_interval(mut self, print_interval: Time) -> Self {
        self.print_interval = Some(print_interval);
        self
    }

    pub fn write(mut self, write: &'a str) -> Self {
        self.write = Some(write);
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
            Some(Output::new(self.output, self.print_interval, self.write))
        }
    }
}
