use clap::{ App, Arg, SubCommand, ArgMatches };

use crate::settings::meta::*;
use crate::settings::args;
use crate::error::*;

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new(NAME)
        .version(VERSION)
        .about(ABOUT)
        .arg(Arg::with_name("time")
             .help(args::time::HELP)
             .long_help(args::time::LONG_HELP)
             .required(true)
             .index(1)
             .multiple(true))
        .get_matches()
}
