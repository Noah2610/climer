use super::Output;
use super::Timer;
use crate::error::ClimerResult;
use crate::time::Time;

pub struct TimerBuilder {
    time:           String,
    quiet:          bool,
    format:         Option<String>,
    output_format:  Option<String>,
    print_interval: Option<Time>,
    write:          Option<String>,
}

impl TimerBuilder {
    pub fn new<T>(time: T) -> Self
    where
        T: ToString,
    {
        Self {
            time:           time.to_string(),
            quiet:          false,
            format:         None,
            output_format:  None,
            print_interval: None,
            write:          None,
        }
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.format = Some(format.to_string());
        self
    }

    pub fn print_interval(mut self, print_interval: Time) -> Self {
        self.print_interval = Some(print_interval);
        self
    }

    pub fn write<T>(mut self, write: T) -> Self
    where
        T: ToString,
    {
        self.write = Some(write.to_string());
        self
    }

    pub fn build(self) -> ClimerResult<Timer> {
        let time = self.time.clone();
        let format = self.format.clone();
        let output = self.build_output();
        Ok(Timer::new(time, format, output)?)
    }

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
