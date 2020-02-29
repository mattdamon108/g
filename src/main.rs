#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::process;

use g::*;

fn main() {
    let matches = App::new("g")
        .author("moondaddi <woonki.moon@gmail.com>")
        .version(crate_version!())
        .about("a tool to manage multiple git remote accounts")
        .arg(
            Arg::with_name("profile")
                .short("p")
                .long("profile")
                .value_name("PROFILE")
                .help("Swap an account with profile"),
        )
        .get_matches();

    let config = Config::new(matches).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);

        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Unexpected error {}", e);

        process::exit(1);
    }
}
