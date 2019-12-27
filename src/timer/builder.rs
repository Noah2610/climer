use super::Output;
use super::Timer;
use crate::error::ClimerResult;
use crate::time::parser::parse_time;
use crate::time::Time;

/// The builder struct for `Timer`.
#[derive(Default)]
pub struct TimerBuilder {
    time:           Option<Time>,
    time_str:       Option<String>,
    start_time:     Option<Time>,
    quiet:          bool,
    format:         Option<String>,
    output_format:  Option<String>,
    print_interval: Option<Time>,
    write:          Option<String>,
}

impl TimerBuilder {
    /// Set the `time` as a `Time`.
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }

    /// Set the `time` as a string.
    pub fn time_str<T>(mut self, time_str: T) -> Self
    where
        T: ToString,
    {
        self.time_str = Some(time_str.to_string());
        self
    }

    /// Set the `start_time` `Time`.
    pub fn start_time(mut self, start_time: Time) -> Self {
        self.start_time = Some(start_time);
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
        let time = if let Some(time) = self.time {
            Some(time)
        } else if let Some(time_str) = self.time_str.as_ref() {
            Some(parse_time(time_str, self.format.as_ref())?)
        } else {
            None
        };

        let start_time = self.start_time;

        let output = self.build_output();
        let mut timer = Timer::new(time, output);

        if let Some(start_time) = start_time {
            timer.set_start_time(start_time);
        }

        Ok(timer)
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
