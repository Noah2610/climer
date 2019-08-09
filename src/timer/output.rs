use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use crate::error::{ClimerError, ClimerResult};
use crate::helpers::*;
use crate::settings::output::*;
use crate::time::prelude::*;

pub struct Output<'a> {
    format:         &'a str,
    write_to_file:  Option<&'a str>,
    print_interval: Time,
    last_print:     Instant,
}

impl<'a> Output<'a> {
    pub fn new(
        format: Option<&'a str>,
        print_interval: Option<Time>,
        write_to_file: Option<&'a str>,
    ) -> Self {
        Self {
            format: format.unwrap_or(DEFAULT_FORMAT),
            write_to_file,
            print_interval: print_interval.unwrap_or(
                TimeBuilder::new()
                    .milliseconds(DEFAULT_PRINT_INTERVAL_MS)
                    .build(),
            ),
            last_print: Instant::now(),
        }
    }

    pub fn update(&mut self, to_print: &str) -> ClimerResult {
        let now = Instant::now();
        if now - self.last_print
            < Duration::from_millis(self.print_interval.as_milliseconds() as u64)
        {
            return Ok(());
        }
        self.print(to_print)?;
        self.last_print = now;
        Ok(())
    }
    pub fn print(&mut self, to_print: &str) -> ClimerResult {
        if let Some(file) = &self.write_to_file {
            self.print_to_file(to_print, file)?;
        } else {
            self.print_to_stdout(to_print)?;
        }
        Ok(())
    }

    fn print_to_stdout(&self, to_print: &str) -> ClimerResult {
        print!("\r{}", to_print);
        flush_stdout()
    }

    fn print_to_file(&self, to_print: &str, file: &'a str) -> ClimerResult {
        let mut buffer = File::create(file)?;
        buffer.write_all(to_print.as_bytes())?;
        buffer.flush()?;

        Ok(())
    }
}
