extern crate clap;

#[macro_use]
mod macros;
mod settings;
mod error;
mod cli;
mod timer;

use self::error::*;
use self::timer::{ Timer, TimerBuilder };

pub fn run() -> ClimerResult {

    let matches = cli::parse();
    //let time: &str = matches.value_of("time").unwrap();
    if let Some(times) = matches.values_of("time") {
        let timer = TimerBuilder::new(&times.collect::<String>())
            .build()?;
        //timer.run()?;
    } else {

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
