extern crate clap;
#[macro_use]
extern crate climer_derive;

#[macro_use]
mod macros;
mod settings;
mod error;
mod cli;
mod time;
mod timer;

use self::error::*;
use self::timer::{ Timer, TimerBuilder };

pub fn run() -> ClimerResult {
    let matches = cli::parse();
    if let Some(times) = matches.values_of("time") {
        let time = &times.collect::<String>();
        let mut builder = TimerBuilder::new(time);
        if let Some(format) = matches.value_of("format") {
            builder = builder.format(format);
        }
        let mut timer = builder.build()?;
        timer.run()?;
    }
    Ok(())
}

use std::io::{ self, Write };
use std::thread::sleep;
use std::time::Duration;

fn stdout_test() {
    for i in 1 ..= 10 {
        print!("\r");
        print!("{}", i);
        io::stdout().flush();
        sleep(Duration::new(1, 0));
    }
}
