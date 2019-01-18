use std::time::{ Instant, Duration };

use crate::settings::output::*;
use crate::error::{ ClimerResult, ClimerError };
use crate::time::prelude::*;

pub struct Output<'a> {
    format:         &'a str,
    print_interval: Time,
    last_print:     Instant,
}

impl<'a> Output<'a> {
    pub fn new(format: &'a str) -> Self {
        Self {
            format,
            print_interval: TimeBuilder::new().milliseconds(DEFAULT_PRINT_INTERVAL_MS).build(),
            last_print:     Instant::now(),
        }
    }

    pub fn new_with_default() -> Self {
        Self::new( DEFAULT_FORMAT )
    }

    pub fn update(&mut self) -> ClimerResult {
        Ok(())
    }

    fn print(&mut self) -> ClimerResult {
        let now = Instant::now();
        //if now - self.last_print < Duration::from_millis(self.print_interval.milliseconds()) {
        Ok(())
    }
}
