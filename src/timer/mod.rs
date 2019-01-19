mod builder;
mod output;

use std::time::{ Instant, Duration };

pub use self::builder::TimerBuilder;
pub use self::output::Output;
use crate::time::{ Time, TimeBuilder };
use crate::settings::timer::*;
use crate::error::ClimerResult;
use crate::time::parser::parse_time;

pub struct Timer<'a> {
    target_time: Time,
    time:        Time,
    output:      Option<Output<'a>>,
    running:     bool,
    last_update: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(time: &str, format: Option<&str>, output: Option<Output<'a>>) -> ClimerResult<Self> {
        Ok(Self {
            target_time: parse_time(time, format)?,
            time:        Time::zero(),
            output,
            running:     false,
            last_update: Instant::now(),
        })
    }

    pub fn run(&mut self) -> ClimerResult {
        self.running = true;
        self.last_update = Instant::now();
        while self.running {
            self.update()?;
        }
        Ok(())
    }

    fn update(&mut self) -> ClimerResult {
        let now = Instant::now();
        let time_output = self.time_output();
        if let Some(output) = &mut self.output {
            output.update(&time_output)?;
        }
        let duration = now.duration_since(self.last_update);
        let time_since = Time::from(duration);
        self.time += time_since;
        self.check_finished()?;
        self.last_update = now;
        Ok(())
    }

    fn check_finished(&mut self) -> ClimerResult {
        if self.time >= self.target_time {
            let time_output = self.time_output();
            if let Some(output) = &mut self.output {
                //output.print(&time_output)?;
                println!("\n{}", FINISH_TEXT);
            }
            self.running = false;
        }
        Ok(())
    }

    fn time_output(&self) -> Time {
        if self.target_time < self.time {
            return Time::zero();
        };
        let ret = self.target_time - self.time;
        //println!("{}", ret);
        ret
    }
}
