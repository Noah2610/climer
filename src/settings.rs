pub mod meta {
    pub const NAME:    &'static str = "Climer";
    pub const VERSION: &'static str = "0.0.0";
    pub const AUTHOR:  &'static str = "Noah Rosenzweig <rosenzweig.noah@gmail.com>";
    pub const ABOUT:   &'static str = "Simple command-line timer program.";
}

pub mod args {
    pub mod time {
        pub const HELP:      &'static str = "Time input as string";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR TIME";
    }
}

pub mod timer {
}

pub mod parser {
    pub mod codes {
        pub const HOUR:        char = 'H';
        pub const MINUTE:      char = 'M';
        pub const SECOND:      char = 'S';
        pub const MILLISECOND: char = 'm';
    }
}
