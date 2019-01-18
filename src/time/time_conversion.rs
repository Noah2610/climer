pub trait TimeConversion {
    fn h(&self)  -> u32;
    fn m(&self)  -> u32;
    fn s(&self)  -> u32;
    fn ms(&self) -> u32;
    fn set_h(&mut self, h: u32);
    fn set_m(&mut self, m: u32);
    fn set_s(&mut self, s: u32);
    fn set_ms(&mut self, ms: u32);

    fn add_h(&mut self, h: u32) {
        self.set_h(self.h() + h);
    }
    fn add_m(&mut self, m: u32) {
        self.set_m(self.m() + m);
    }
    fn add_s(&mut self, s: u32) {
        self.set_s(self.s() + s);
    }
    fn add_ms(&mut self, ms: u32) {
        self.set_ms(self.ms() + ms);
    }

    fn add_hours(&mut self, hours: u32) {
        self.add_h(hours);
    }

    fn add_minutes(&mut self, minutes: u32) {
        let hours = minutes / 60;
        self.add_hours(hours);
        self.add_m(minutes - hours * 60);
    }

    fn add_seconds(&mut self, seconds: u32) {
        let minutes = seconds / 60;
        self.add_minutes(minutes);
        self.add_s(seconds - minutes * 60);
    }

    fn add_milliseconds(&mut self, milliseconds: u32) {
        let seconds = milliseconds / 1000;
        self.add_seconds(seconds);
        self.add_ms(milliseconds - seconds * 1000);
    }
}
