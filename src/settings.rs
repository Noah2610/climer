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

    pub mod format {
        pub const HELP:      &'static str = "Format for time input as string";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR FORMAT";
    }

    pub mod output {
        pub const HELP:      &'static str = "Custom output format during timer";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR OUTPUT";
    }

    pub mod quiet {
        pub const HELP:      &'static str = "Quiet; don't print anything to stdout";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR QUIET";
    }
}

pub mod timer {
    pub const FINISH_TEXT: &'static str = "Time's Up!";
}

pub mod parser {
    pub mod codes {
        pub const HOUR:        &'static str = "h";
        pub const MINUTE:      &'static str = "m";
        pub const SECOND:      &'static str = "s";
        pub const MILLISECOND: &'static str = "ms";
        pub const NANOSECOND:  &'static str = "ns";
    }
}

pub mod output {
    pub const DEFAULT_FORMAT: &'static str = "%H:%M:%S";
    pub const DEFAULT_PRINT_INTERVAL_MS: u64 = 10;
}
