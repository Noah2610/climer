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
