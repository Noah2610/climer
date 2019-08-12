use super::Output;
use super::Timer;
use crate::error::{ClimerError, ClimerResult};
use crate::time::parser::parse_time;
use crate::time::Time;

/// The builder struct for `Timer`.
#[derive(Default)]
pub struct TimerBuilder {
    time:           Option<String>,
    quiet:          bool,
    format:         Option<String>,
    output_format:  Option<String>,
    print_interval: Option<Time>,
    write:          Option<String>,
}

impl TimerBuilder {
    /// Set the `time` as a string.
    pub fn time<T>(mut self, time: T) -> Self
    where
        T: ToString,
    {
        self.time = Some(time.to_string());
        self
    }

    /// Set the `quiet` flag, used for printing to stdout.
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    /// TODO: Unimplemented
    /// Set the `format` string, used for parsing the given time string.
    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.format = Some(format.to_string());
        self
    }

    /// Set the interval in which output should be printed to stdout.
    pub fn print_interval(mut self, print_interval: Time) -> Self {
        self.print_interval = Some(print_interval);
        self
    }

    /// Set a file for the output to be written to, instead of stdout.
    pub fn write<T>(mut self, write: T) -> Self
    where
        T: ToString,
    {
        self.write = Some(write.to_string());
        self
    }

    /// Build a `Timer`.
    pub fn build(self) -> ClimerResult<Timer> {
        let time = if let Some(time) = self.time.as_ref() {
            Some(parse_time(time, self.format.as_ref())?)
        } else {
            None
        };
        let output = self.build_output();
        Ok(Timer::new(time, output)?)
    }

    /// Build the `Output` used by the `Timer`.
    fn build_output(self) -> Option<Output> {
        if self.quiet {
            None
        } else {
            Some(Output::new(
                self.output_format,
                self.print_interval,
                self.write,
            ))
        }
    }
}
