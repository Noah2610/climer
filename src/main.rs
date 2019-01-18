extern crate climer;

use std::process::exit;

fn main() {
    if let Err(err) = climer::run() {
        eprintln!("An error occured:\n{}", err);
        exit(1);
    }
}
