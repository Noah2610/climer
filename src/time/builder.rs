use super::{ Time, TimeConversion };

#[derive(TimeConversion)]
pub struct TimeBuilder {
    hours:        u32,
    minutes:      u32,
    seconds:      u32,
    milliseconds: u32,
}

impl TimeBuilder {
    pub fn new() -> Self {
        Self { hours: 0, minutes: 0, seconds: 0, milliseconds: 0 }
    }

    pub fn hours(mut self, hours: u32) -> Self {
        self.hours = 0;
        self.add_hours(hours);
        self
    }

    pub fn minutes(mut self, minutes: u32) -> Self {
        self.minutes = 0;
        self.add_minutes(minutes);
        self
    }

    pub fn seconds(mut self, seconds: u32) -> Self {
        self.seconds = 0;
        self.add_seconds(seconds);
        self
    }

    pub fn milliseconds(mut self, milliseconds: u32) -> Self {
        self.milliseconds = 0;
        self.add_milliseconds(milliseconds);
        self
    }

    pub fn build(self) -> Time {
        Time::new(
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::Time;
    use super::TimeBuilder;

    fn get_expected_time() -> Time {
        Time {
            hours:        1,
            minutes:      10,
            seconds:      20,
            milliseconds: 500,
        }
    }

    #[test]
    fn builds_time() {
        let expected = get_expected_time();
        let actual   = TimeBuilder::new()
            .hours(1)
            .minutes(10)
            .seconds(20)
            .milliseconds(500)
            .build();
        assert_eq!(actual, expected);
    }

    fn handles_overflow() {
        let expected = get_expected_time();
        let actual   = TimeBuilder::new()
            //                   hours               minutes          seconds  milliseconds
            .milliseconds((1 * 60 * 60 * 1000) + (10 * 60 * 1000) + (20 * 1000) + 500)
            .build();
        assert_eq!(actual, expected);
    }
}
