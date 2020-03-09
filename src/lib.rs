extern crate clap;

use clap::ArgMatches;
use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
pub struct Config {
    pub profile: Option<String>,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, Box<Error + Send + Sync>> {
        let profile = match matches.value_of("profile") {
            Some(p) => Some(p.to_string()),
            _ => None,
        };

        Ok(Config { profile })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let git_config = GitConfig::new();

    match config.profile {
        Some(p) => {
            // do something with profile as args
            println!("{}", p)
        }
        None => {
            // do something without profile
            match git_config {
                Ok(c) => {
                    if !c.has_credential {
                        print!("You didnt't config credentials before. Will you configure to store and use credentials? (y/N)");
                        stdout().flush().unwrap();
                        let mut input = [0];
                        if let Ok(i) = stdin().read(&mut input) {
                            if i <= 1 {
                                println!("Input {}", input[0])
                            }
                        }
                    }
                }
                _ => println!("Error: cannot find .gitconfig file in your system"),
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct GitConfig {
    pub has_credential: bool,
    pub has_store: bool,
    pub store_file: Option<String>,
}

impl GitConfig {
    pub fn new() -> Result<GitConfig, Box<dyn Error + Send + Sync>> {
        let gitconfig = fs::read_to_string("./test/.gitconfig_test")?;

        let mut has_credential: bool = false;
        let mut has_store: bool = false;
        let mut store_file: Option<String> = None;

        for (count, line) in gitconfig.lines().enumerate() {
            let trimed_line = line.trim();

            if trimed_line == "[credential]" {
                has_credential = true;
                continue;
            }

            if has_credential & trimed_line.contains("helper") & trimed_line.contains("store") {
                has_store = true;

                if trimed_line.contains("--file") {
                    match trimed_line.find("--file") {
                        Some(index) => {
                            store_file = Some(trimed_line[index + 7..].trim().to_string())
                        }
                        None => store_file = None,
                    }
                }
            }
        }

        Ok(GitConfig {
            has_credential,
            has_store,
            store_file,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
