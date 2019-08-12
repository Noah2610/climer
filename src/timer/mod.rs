mod builder;
pub mod output;

use std::thread::sleep;
use std::time::{Duration, Instant};

pub use self::builder::TimerBuilder;
pub use self::output::Output;
use crate::error::{ClimerError, ClimerResult};
use crate::settings::timer::*;
use crate::time::Time;

pub struct Timer {
    pub running:     bool,
    pub finished:    bool,
    target_time:     Option<Time>,
    time:            Time,
    output:          Option<Output>,
    update_delay_ms: u64,
    last_update:     Option<Instant>,
}

impl Timer {
    /// Returns a new `TimerBuilder`.
    pub fn builder() -> TimerBuilder {
        TimerBuilder::default()
    }

    /// Create a new `Timer` with the given optional arguments:
    /// `target_time` (`Time`), and `output` (`Output`).
    /// If a `target_time` is given, then the timer will act more like a _countdown_.
    /// If no `target_time` is given, then the timer will act more like a _stopwatch_;
    /// it will never finish naturally, so instead you need to stop the timer when necessary.
    pub fn new(target_time: Option<Time>, output: Option<Output>) -> Self {
        Self {
            running: false,
            finished: false,
            target_time,
            time: Time::zero(),
            output,
            update_delay_ms: 100, // TODO
            last_update: None,
        }
    }

    /// Run the timer.
    /// This will lock the current thread until the time is up.
    /// If you want to update the timer yourself somewhere else,
    /// then don't call this method, instead call the `update` method to update the timer.
    pub fn run(&mut self) -> ClimerResult {
        self.start()?;
        while self.running {
            self.update()?;
            sleep(Duration::from_millis(self.update_delay_ms))
        }
        Ok(())
    }

    /// Start the timer. Do not call this method directly if you are using the `run` method.
    /// Only call this method if you intend to update the timer manually
    /// by calling the `update` method.
    pub fn start(&mut self) -> ClimerResult {
        if self.running {
            return Err(ClimerError::TimerAlreadyRunning);
        }
        self.running = true;
        self.finished = false;
        let now = Instant::now();
        self.last_update = Some(now);
        Ok(())
    }

    /// Stops the timer, after it has been started using the `start` method.
    pub fn stop(&mut self) -> ClimerResult {
        if !self.running {
            return Err(ClimerError::TimerNotRunning);
        }
        self.running = false;
        self.last_update = None;
        Ok(())
    }

    /// Updates the timer.
    /// This will also attempt to write the remaining time to stdout or to a file,
    /// using the `Output` of this `Timer`.
    pub fn update(&mut self) -> ClimerResult {
        if !self.running {
            return Err(ClimerError::TimerNotRunning);
        }

        let now = Instant::now();
        self.print_output()?;
        let duration = now.duration_since(self.last_update.unwrap_or(now));
        let time_since = Time::from(duration);
        self.time += time_since;
        if self.target_time.is_some() {
            self.check_finished()?;
        }
        self.last_update = Some(now);
        Ok(())
    }

    /// Print the current time to stdout or to a file using this timer's `Output`
    /// (if output exists).
    pub fn print_output(&mut self) -> ClimerResult {
        let time_output = self.time_output();
        if let Some(output) = &mut self.output {
            output.update(format!("{}", time_output))
        } else {
            Ok(())
        }
    }

    /// Returns a `Time`.
    /// If a `target_time` was given, then it returns the _remaining time_
    /// until the timer finishes;
    /// if no `target_time` was given, then it returns the time since the timer was started.
    pub fn time_output(&self) -> Time {
        if let Some(target_time) = self.target_time {
            if target_time < self.time {
                return Time::zero();
            };
            target_time - self.time
        } else {
            self.time
        }
    }

    /// Check if the timer is finished.
    /// Returns a `ClimerError` if the timer isn't running, or no `target_time` was given.
    fn check_finished(&mut self) -> ClimerResult {
        if !self.running {
            return Err(ClimerError::TimerNotRunning);
        }

        if let Some(target_time) = self.target_time {
            if self.time >= target_time {
                //let time_output = self.time_output();
                if let Some(output) = &mut self.output {
                    //output.print(&format!("{}", time_output))?;
                    output.print(FINISH_TEXT)?;
                }
                self.finish()?;
            }
            Ok(())
        } else {
            Err(ClimerError::TimerCannotFinish)
        }
    }

    /// Finish the timer. This method should only be called from `check_finished`, which verifies
    /// that the timer has actually finished.
    fn finish(&mut self) -> ClimerResult {
        self.finished = true;
        self.stop()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(None, None)
    }
}
