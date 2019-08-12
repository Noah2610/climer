mod settings;

use clap::{App, Arg, ArgMatches};

use settings::args;
use settings::meta::*;

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new(NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT)
        .after_help(AFTER_HELP)
        .arg(
            Arg::with_name("time")
                .help(args::time::HELP)
                .long_help(args::time::LONG_HELP)
                .required(false)
                .index(1)
                .multiple(true),
        )
        .arg(
            Arg::with_name("quiet")
                .help(args::quiet::HELP)
                .long_help(args::quiet::LONG_HELP)
                .short("q")
                .long("quiet"),
        )
        .arg(
            Arg::with_name("format") // TODO
                .help(args::format::HELP)
                .long_help(args::format::LONG_HELP)
                .short("f")
                .long("format")
                .takes_value(true),
        )
        // .arg(
        //     Arg::with_name("output") // TODO
        //         .help(args::output::HELP)
        //         .long_help(args::output::LONG_HELP)
        //         .short("o")
        //         .long("output")
        //         .takes_value(true),
        // )
        .arg(
            Arg::with_name("print_interval")
                .help(args::print_interval::HELP)
                .long_help(args::print_interval::LONG_HELP)
                .short("i")
                .long("interval")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("write")
                .help(args::write::HELP)
                .long_help(args::write::LONG_HELP)
                .short("w")
                .long("write")
                .takes_value(true),
        )
        .get_matches()
}
