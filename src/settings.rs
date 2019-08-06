pub mod meta {
    pub const NAME: &'static str = "climer";
    pub const ABOUT: &'static str = "Simple command-line timer app.";
}

pub mod args {
    pub mod time {
        pub const HELP: &'static str = "Time input as string.";
        pub const LONG_HELP: &'static str = "Time input as semantic string.
The input follows the pattern of
  `<NUM><UNIT>`
where <NUM> can be any number, and <UNIT> is a time unit such as
  `h`  for _hours_
  `m`  for _minutes_
  `s`  for _seconds_
  `ms` for _milliseconds_
  `ns` for _nanoseconds_";
    }

    pub mod quiet {
        pub const HELP: &'static str = "Quiet; don't print anything to stdout.";
        pub const LONG_HELP: &'static str = HELP;
    }

    pub mod format {
        pub const HELP: &'static str =
            "(UNIMPLEMENTED) Format for time input as string.";
        pub const LONG_HELP: &'static str = HELP;
    }

    pub mod output {
        pub const HELP: &'static str = "Custom output format during timer.";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR OUTPUT";
    }

    pub mod print_interval {
        pub const HELP: &'static str =
            "Interval in milliseconds between prints";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR PRINT_INTERVAL";
    }

    pub mod write {
        pub const HELP: &'static str = "Write time to file instead of stdout";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR WRITE";
    }
}

pub mod timer {
    pub const FINISH_TEXT: &'static str = "Time's Up!";
}

pub mod parser {
    pub mod codes {
        pub const HOUR: &'static str = "h";
        pub const MINUTE: &'static str = "m";
        pub const SECOND: &'static str = "s";
        pub const MILLISECOND: &'static str = "ms";
        pub const NANOSECOND: &'static str = "ns";
    }
}

pub mod output {
    pub const DEFAULT_FORMAT: &'static str = "%H:%M:%S";
    pub const DEFAULT_PRINT_INTERVAL_MS: u64 = 100;
}
