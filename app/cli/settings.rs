pub mod meta {
    pub const NAME: &'static str = "climer";
    pub const ABOUT: &'static str = "Simple command-line timer app.";
    pub const AFTER_HELP: &'static str = "\
EXAMPLES:
    Set a timer for 2 minutes and 10 seconds:
        climer 2m10s
    Set a timer which only updates every second and writes its output to \
                                          `./remaining_time`.
        climer -i 1000 1h 30m > ./remaining_time
    Print the output to a file, play an audio file when the timer finishes,
    and disown the climer process, so we can close the shell.
    We can then read from the output file somewhere else to display the \
                                          remaining time.
        climer -i 500 2m30s > $HOME/.remaining_time && \\
            mpv $HOME/Music/alarm.mp3 & \\
            disown
        # Now we could check the time from anywhere by reading from \
                                          ~/.remaining_time
        tail -f $HOME/.remaining_time

NOTE:
    Although the --write feature works, it doesn't seem to update the fs quite \
                                          right.
    Not sure what's wrong, but when using `tail -f` for example, tail would \
                                          take some seconds before
    reading from the file again.
    Currently, it works better to just redirect the stdout directly from your \
                                          shell, for example:
        climer -i 500 2m > $HOME/.remaining_time &";
}

pub mod args {
    pub mod time {
        pub const HELP: &'static str =
            "Time input as string. Acts like a stopwatch if omitted.";
        pub const LONG_HELP: &'static str = "\
Time input as semantic string.
The input follows the pattern of
  `<NUM><UNIT>`
where <NUM> can be any number, and <UNIT> is a time unit such as
  `h`  for _hours_
  `m`  for _minutes_
  `s`  for _seconds_
  `ms` for _milliseconds_
  `ns` for _nanoseconds_

If this argument is omitted, then the timer will act like a stopwatch.
This means, the timer will count upwards and never finish naturally.";
    }

    pub mod quiet {
        pub const HELP: &'static str = "Quiet; don't print anything to stdout.";
        pub const LONG_HELP: &'static str = HELP;
    }

    pub mod arg_continue {
        pub const HELP: &'static str =
            "Continue running timer after it finishes.";
        pub const LONG_HELP: &'static str = "Continue running timer into negative time after the timer finishes.\n\
            The timer will run endlessly with this flag set.\n\
            Does nothing when no target time is given.";
    }

    pub mod format {
        pub const HELP: &'static str =
            "(UNIMPLEMENTED) Format for time input as string.";
        pub const LONG_HELP: &'static str = HELP;
    }

    pub mod output {
        // TODO: Unimplemented
        pub const HELP: &'static str = "Custom output format during timer.";
        pub const LONG_HELP: &'static str = "LONG_HELP FOR OUTPUT";
    }

    pub mod print_interval {
        pub const HELP: &'static str =
            "Interval in milliseconds between prints.";
        pub const LONG_HELP: &'static str = "The amount of time in \
                                             milliseconds\nthat the program \
                                             sleeps between updates.";
    }

    pub mod write {
        pub const HELP: &'static str =
            "Write output to file instead of stdout.";
        pub const LONG_HELP: &'static str = "\
Write output to file instead of stdout.
NOTE: This doesn't quite work correctly yet,
see the note at the bottom of the help screen for more info.
For now, it is recommended to just redirect the stdout from your shell.";
    }

    pub mod start_time {
        pub const HELP: &'static str = "Set the starting time for the timer";
        pub const LONG_HELP: &'static str =
            "Set the starting time for the timer.\nDefaults to _zero_";
    }
}
