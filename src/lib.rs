#[macro_use]
extern crate clap;
extern crate regex;
#[macro_use]
extern crate climer_derive;

mod settings;
#[macro_use]
pub mod macros;
pub mod cli;
pub mod error;
pub mod helpers;
pub mod time;
pub mod timer;
