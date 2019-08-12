mod builder;
pub mod output;

use std::thread::sleep;
use std::time::{Duration, Instant};

pub use self::builder::TimerBuilder;
pub use self::output::Output;
use crate::error::ClimerResult;
use crate::settings::timer::*;
use crate::time::parser::parse_time;
use crate::time::Time;

pub struct Timer {
    target_time:     Time,
    time:            Time,
    output:          Option<Output>,
    running:         bool,
    update_delay_ms: u64,
    last_update:     Instant,
}

impl Timer {
    pub fn new<T, U>(
        time: T,
        format: Option<U>,
        output: Option<Output>,
    ) -> ClimerResult<Self>
    where
        T: ToString,
        U: ToString,
    {
        Ok(Self {
            target_time: parse_time(time, format)?,
            time: Time::zero(),
            output,
            running: false,
            update_delay_ms: 100, // TODO
            last_update: Instant::now(),
        })
    }

    pub fn run(&mut self) -> ClimerResult {
        self.running = true;
        self.last_update = Instant::now();
        while self.running {
            self.update()?;
            sleep(Duration::from_millis(self.update_delay_ms))
        }
        Ok(())
    }

    fn update(&mut self) -> ClimerResult {
        let now = Instant::now();
        let time_output = self.time_output();
        if let Some(output) = &mut self.output {
            output.update(format!("{}", time_output))?;
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
            //let time_output = self.time_output();
            if let Some(output) = &mut self.output {
                //output.print(&format!("{}", time_output))?;
                output.print(FINISH_TEXT)?;
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
        ret
    }
}
