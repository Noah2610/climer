pub mod output;

mod builder;
mod state;

pub use builder::TimerBuilder;
pub use output::Output;
pub use state::TimerState;

use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::error::{ClimerError, ClimerResult};
use crate::settings::timer::*;
use crate::time::Time;

#[derive(Clone)]
pub struct Timer {
    pub state: TimerState,
    target_time: Option<Time>,
    time: Time,
    start_time: Option<Time>,
    continue_after_finish: bool,
    output: Option<Output>,
    update_delay_ms: u64,
    last_update: Option<Instant>,
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
            state: TimerState::Stopped,
            target_time,
            time: Time::zero(),
            start_time: None,
            continue_after_finish: false,
            output,
            update_delay_ms: 100, // TODO
            last_update: None,
        }
    }

    /// Set a new target `Time`.
    pub fn set_target_time(&mut self, target_time: Time) {
        self.target_time = Some(target_time);
    }

    pub fn set_continue_after_finish(&mut self, continue_after_finish: bool) {
        self.continue_after_finish = continue_after_finish;
    }

    /// Removes the target `Time`.
    /// If the timer has no target time, then it acts like a stopwatch,
    /// endlessly counting upwards.
    pub fn clear_target_time(&mut self) {
        self.target_time = None;
    }

    /// Set the starting `time` value.
    /// Used for starting the timer with a starting time other than 0.
    pub fn set_start_time(&mut self, start_time: Time) {
        self.start_time = Some(start_time);
    }

    /// Removes the start `Time`.
    pub fn clear_start_time(&mut self) {
        self.start_time = None;
    }

    /// Run the timer.
    /// This will lock the current thread until the time is up.
    /// If you want to update the timer yourself somewhere else,
    /// then don't call this method, instead call the `update` method to update the timer.
    pub fn run(&mut self) -> ClimerResult {
        self.start()?;
        while self.state.is_running() {
            self.update()?;
            sleep(Duration::from_millis(self.update_delay_ms))
        }
        Ok(())
    }

    /// Start the timer. Do not call this method directly if you are using the `run` method.
    /// Only call this method if you intend to update the timer manually
    /// by calling the `update` method.
    /// The `start` method can also be used as a _restart_.
    pub fn start(&mut self) -> ClimerResult {
        self.time = self
            .start_time
            .as_ref()
            .map(Clone::clone)
            .unwrap_or_else(Time::zero);
        self.state = TimerState::Running;
        let now = Instant::now();
        self.last_update = Some(now);
        Ok(())
    }

    /// Stops the timer, after it has been started using the `start` method.
    pub fn stop(&mut self) -> ClimerResult {
        self.state = TimerState::Stopped;
        self.last_update = None;
        Ok(())
    }

    /// Pauses the timer.
    pub fn pause(&mut self) -> ClimerResult {
        match &self.state {
            TimerState::Running => {
                self.update()?;
                self.last_update = None;
                self.state = TimerState::Paused;
                Ok(())
            }
            _ => Err(ClimerError::TimerNotRunning),
        }
    }

    /// Resumes the timer from a paused state.
    pub fn resume(&mut self) -> ClimerResult {
        match &self.state {
            TimerState::Paused => {
                self.last_update = Some(Instant::now());
                self.update()?;
                self.state = TimerState::Running;
                Ok(())
            }
            _ => Err(ClimerError::TimerAlreadyRunning),
        }
    }

    /// Updates the timer.
    /// This will also attempt to write the remaining time to stdout or to a file,
    /// using the `Output` of this `Timer`.
    pub fn update(&mut self) -> ClimerResult {
        match &self.state {
            TimerState::Running => {
                let now = Instant::now();
                self.print_output()?;
                let duration =
                    now.duration_since(self.last_update.unwrap_or(now));
                let time_since = Time::from(duration);
                self.time += time_since;

                if self.target_time.is_some() {
                    self.check_finished()?;
                }

                self.last_update = Some(now);
                Ok(())
            }
            _ => Ok(()),
        }
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
        if !self.state.is_running() {
            return Err(ClimerError::TimerNotRunning);
        }

        if let Some(target_time) = self.target_time {
            if self.time >= target_time {
                if self.continue_after_finish {
                    self.last_update = Some(Instant::now());
                    self.target_time = None;
                    self.time = Time::zero();
                    if let Some(output) = &mut self.output {
                        output.set_prefix(Some("-".to_string()));
                    }
                } else {
                    if let Some(output) = &mut self.output {
                        output.print(FINISH_TEXT)?;
                    }
                    self.finish_without_update()?;
                }
            }
            Ok(())
        } else {
            Err(ClimerError::TimerCannotFinish)
        }
    }

    /// Finish the timer.
    pub fn finish(&mut self) -> ClimerResult {
        self.update()?; // Update one last time, to get a correct final time
        self.finish_without_update()
    }

    /// Finish the timer without updating one final time.
    /// This is only used internally from the `check_finished` method,
    /// to avoid recursion.
    fn finish_without_update(&mut self) -> ClimerResult {
        self.stop()?;
        self.state = TimerState::Finished;
        Ok(())
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(None, None)
    }
}
