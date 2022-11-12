use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use crate::error::ClimerResult;
use crate::helpers::*;
use crate::settings::output::*;
use crate::time::prelude::*;

#[derive(Clone)]
pub struct Output {
    format: String,
    write_to_file: Option<String>,
    print_interval: Time,
    last_print: Instant,
    prefix: Option<String>,
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
            format: format
                .map(|s| s.to_string())
                .unwrap_or(DEFAULT_FORMAT.to_string()),
            write_to_file: write_to_file.map(|s| s.to_string()),
            print_interval: print_interval.unwrap_or(
                TimeBuilder::new()
                    .milliseconds(DEFAULT_PRINT_INTERVAL_MS)
                    .build(),
            ),
            last_print: Instant::now(),
            prefix: None,
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

    pub fn set_prefix(&mut self, prefix: Option<String>) {
        self.prefix = prefix;
    }

    fn print_to_stdout<T>(&self, to_print: T) -> ClimerResult
    where
        T: ToString,
    {
        print!("\r{}", self.format_to_print(to_print));
        flush_stdout()
    }

    fn print_to_file<T, U>(&self, to_print: T, file: U) -> ClimerResult
    where
        T: ToString,
        U: ToString,
    {
        let mut buffer = File::create(file.to_string())?;
        buffer.write_all(self.format_to_print(to_print).as_bytes())?;
        buffer.flush()?;

        Ok(())
    }

    fn format_to_print<T>(&self, to_print: T) -> String
    where
        T: ToString,
    {
        if let Some(prefix) = self.prefix.as_ref() {
            format!("{}{}", prefix, to_print.to_string())
        } else {
            to_print.to_string()
        }
    }
}
