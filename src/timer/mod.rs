mod builder;
mod output;
mod parser;

use std::time::Duration;

pub use self::builder::TimerBuilder;
pub use self::output::Output;
use crate::time::{ Time, TimeBuilder };
use crate::settings::timer::*;
use crate::error::ClimerResult;
use self::parser::parse_time;

pub struct Timer<'a> {
    time:    Time,
    output:  Option<Output<'a>>,
    running: bool,
}

impl<'a> Timer<'a> {
    pub fn new(time: &str, format: Option<&str>, output: Option<Output<'a>>) -> ClimerResult<Self> {
        Ok(Self {
            time: parse_time(time, format)?,
            output,
            running: false,
        })
    }

    pub fn run(&mut self) -> ClimerResult {
        self.running = true;
        while self.running {
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> ClimerResult {
        if let Some(output) = &mut self.output {
            output.update(&self.time)?;
        }
        Ok(())
    }
}

