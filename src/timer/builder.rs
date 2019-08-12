use super::Output;
use super::Timer;
use crate::error::ClimerResult;
use crate::time::Time;

pub struct TimerBuilder<'a> {
    time:           &'a str,
    quiet:          bool,
    format:         Option<&'a str>,
    output_format:  Option<&'a str>,
    print_interval: Option<Time>,
    write:          Option<&'a str>,
}

impl<'a> TimerBuilder<'a> {
    pub fn new(time: &'a str) -> Self {
        Self {
            time,
            quiet: false,
            format: None,
            output_format: None,
            print_interval: None,
            write: None,
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

    pub fn build(self) -> ClimerResult<Timer<'a>> {
        Ok(Timer::new(self.time, self.format, self.build_output())?)
    }

    fn build_output(self) -> Option<Output<'a>> {
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
