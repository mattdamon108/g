extern crate clap;

use clap::ArgMatches;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub profile: Option<String>,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, &'static str> {
        let profile = match matches.value_of("profile") {
            Some(p) => Some(p.to_string()),
            _ => None,
        };

        Ok(Config { profile })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.profile {
        Some(p) => {
            // do something with profile as args
            println!("{}", p)
        }
        None => {
            // do something without profile
            panic!("No given profile")
        }
    }

    Ok(())
}
