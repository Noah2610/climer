use super::{ Time, TimeConversion };

#[derive(TimeConversion)]
pub struct TimeBuilder {
    hours:        u64,
    minutes:      u64,
    seconds:      u64,
    milliseconds: u64,
    nanoseconds:  u64,
}

impl TimeBuilder {
    pub fn new() -> Self {
        Self { hours: 0, minutes: 0, seconds: 0, milliseconds: 0, nanoseconds: 0 }
    }

    pub fn hours(mut self, hours: u64) -> Self {
        self.hours = 0;
        self.add_hours(hours);
        self
    }

    pub fn minutes(mut self, minutes: u64) -> Self {
        self.minutes = 0;
        self.add_minutes(minutes);
        self
    }

    pub fn seconds(mut self, seconds: u64) -> Self {
        self.seconds = 0;
        self.add_seconds(seconds);
        self
    }

    pub fn milliseconds(mut self, milliseconds: u64) -> Self {
        self.milliseconds = 0;
        self.add_milliseconds(milliseconds);
        self
    }

    pub fn nanoseconds(mut self, nanoseconds: u64) -> Self {
        self.nanoseconds = 0;
        self.add_nanoseconds(nanoseconds);
        self
    }

    pub fn build(self) -> Time {
        Time::new(
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
            self.nanoseconds
        )
    }
}

#[cfg(test)]
mod tests;
