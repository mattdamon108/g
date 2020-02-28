#[macro_use]
extern crate clap;

use clap::{App, Arg};

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
}
