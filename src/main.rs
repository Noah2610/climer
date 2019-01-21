extern crate climer;

use std::process::exit;

fn main() {
    if let Err(err) = climer::run() {
        eprintln!("ERROR: {}", err);
        exit(1);
    }
}
