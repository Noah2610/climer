#[macro_use]
extern crate clap;
extern crate climer;

mod cli;

use climer::error::*;
use climer::time::parser::parse_time;
use climer::time::TimeBuilder;
use climer::timer::TimerBuilder;

fn main() -> ClimerResult {
    let matches = cli::parse();
    let mut builder = TimerBuilder::default();

    if let Some(times) = matches.values_of("time") {
        let time = &times.collect::<String>();
        builder = builder.time_str(time);
    }

    builder = builder.continue_after_finish(matches.is_present("continue"));

    if let Some(format) = matches.value_of("format") {
        builder = builder.format(format);
    }

    if let Some(print_interval_str) = matches.value_of("print_interval") {
        let print_interval_ms = if let Ok(ms) = print_interval_str.parse() {
            ms
        } else {
            return Err(ClimerError::InvalidPrintIntervalValue(
                print_interval_str.to_string(),
            ));
        };
        builder = builder.print_interval(
            TimeBuilder::new().milliseconds(print_interval_ms).build(),
        );
    }

    if let Some(write) = matches.value_of("write") {
        builder = builder.write(write);
    }

    if let Some(start_time_str) = matches.value_of("start_time") {
        let start_time = parse_time::<_, String>(start_time_str, None)?;
        builder = builder.start_time(start_time);
    }

    builder = builder.quiet(matches.is_present("quiet"));

    let mut timer = builder.build()?;
    timer.run()?;
    Ok(())
}
