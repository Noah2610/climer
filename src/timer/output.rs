use std::time::{ Instant, Duration };
use std::io::{ self, Write };
use std::fs;
use std::process::Command;

use crate::settings::output::*;
use crate::error::{ ClimerResult, ClimerError };
use crate::time::prelude::*;

pub struct Output<'a> {
    format:         &'a str,
    write_to_file:  Option<&'a str>,
    print_interval: Time,
    last_print:     Instant,
}

impl<'a> Output<'a> {
    pub fn new(format: Option<&'a str>, print_interval: Option<Time>, write_to_file: Option<&'a str>) -> Self {
        Self {
            format:         format.unwrap_or(DEFAULT_FORMAT),
            write_to_file,
            print_interval: print_interval.unwrap_or(
                TimeBuilder::new().milliseconds(DEFAULT_PRINT_INTERVAL_MS).build()
            ),
            last_print:     Instant::now(),
        }
    }

    pub fn update(&mut self, to_print: &str) -> ClimerResult {
        let now = Instant::now();
        if now - self.last_print < Duration::from_millis(self.print_interval.as_milliseconds() as u64) {
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

    fn print_to_stdout(&mut self, to_print: &str) -> ClimerResult {
        print!("\r{}", to_print);
        if io::stdout().flush().is_err() {
            return Err(ClimerError::UnknownError("Couldn't flush stdout".to_string()));
        }
        Ok(())
    }

    fn print_to_file(&mut self, to_print: &str, file: &'a str) -> ClimerResult {
        if fs::write("/home/noah/.bar_output", to_print).is_err() {
            return Err(ClimerError::UnknownError(format!("Couldn't write to file '{}'", file).to_string()));
        }
        Ok(())
    }
}
