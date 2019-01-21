extern crate regex;
extern crate clap;
#[macro_use]
extern crate climer_derive;

#[macro_use]
pub mod macros;
pub mod error;
pub mod time;
mod settings;
mod cli;
mod timer;

use self::error::*;
use self::timer::TimerBuilder;

pub fn run() -> ClimerResult {
    let matches = cli::parse();
    if let Some(times) = matches.values_of("time") {
        let time = &times.collect::<String>();
        let mut builder = TimerBuilder::new(time);

        if let Some(format) = matches.value_of("format") {
            builder = builder.format(format);
        }

        if let Some(write) = matches.value_of("write") {
            builder = builder.write(write);
        }

        builder = builder.quiet(matches.is_present("quiet"));

        let mut timer = builder.build()?;
        timer.run()?;
    }
    Ok(())
}
