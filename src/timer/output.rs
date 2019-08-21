use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use crate::error::ClimerResult;
use crate::helpers::*;
use crate::settings::output::*;
use crate::time::prelude::*;

#[derive(Clone)]
pub struct Output {
    format:         String,
    write_to_file:  Option<String>,
    print_interval: Time,
    last_print:     Instant,
}

impl Output {
    pub fn new<T, U>(
        format: Option<T>,
        print_interval: Option<Time>,
        write_to_file: Option<U>,
    ) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self {
            format:         format
                .map(|s| s.to_string())
                .unwrap_or(DEFAULT_FORMAT.to_string()),
            write_to_file:  write_to_file.map(|s| s.to_string()),
            print_interval: print_interval.unwrap_or(
                TimeBuilder::new()
                    .milliseconds(DEFAULT_PRINT_INTERVAL_MS)
                    .build(),
            ),
            last_print:     Instant::now(),
        }
    }

    pub fn update<T>(&mut self, to_print: T) -> ClimerResult
    where
        T: ToString,
    {
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

    pub fn print<T>(&mut self, to_print: T) -> ClimerResult
    where
        T: ToString,
    {
        if let Some(file) = self.write_to_file.as_ref() {
            self.print_to_file(to_print, file)?;
        } else {
            self.print_to_stdout(to_print)?;
        }
        Ok(())
    }

    fn print_to_stdout<T>(&self, to_print: T) -> ClimerResult
    where
        T: ToString,
    {
        print!("\r{}", to_print.to_string());
        flush_stdout()
    }

    fn print_to_file<T, U>(&self, to_print: T, file: U) -> ClimerResult
    where
        T: ToString,
        U: ToString,
    {
        let mut buffer = File::create(file.to_string())?;
        buffer.write_all(to_print.to_string().as_bytes())?;
        buffer.flush()?;

        Ok(())
    }
}
