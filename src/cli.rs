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
        .arg(Arg::with_name("format")
             .help(args::format::HELP)
             .long_help(args::format::LONG_HELP)
             .short("f")
             .long("format")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .help(args::output::HELP)
             .long_help(args::output::LONG_HELP)
             .short("o")
             .long("output")
             .takes_value(true))
        .arg(Arg::with_name("quiet")
             .help(args::quiet::HELP)
             .long_help(args::quiet::LONG_HELP)
             .short("q")
             .long("quiet"))
        .get_matches()
}
