extern crate clap;
extern crate mktemp;
extern crate termion;

use clap::ArgMatches;
use mktemp::Temp;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::io::{copy, stdin, stdout, Read, Write};
use std::path::Path;
use std::convert::TryInto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

#[derive(Debug)]
pub struct Config {
    pub profile: Option<String>,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, Box<dyn Error + Send + Sync>> {
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
                        print!("You haven't configured credentials before. Will you configure to use credentials? (y/N) ");
                        // Y: 89 , y: 121
                        // N: 78 , n: 110
                        stdout().flush().unwrap();
                        let mut buf = String::new();
                        stdin().read_line(&mut buf)?;
                        let trimed_answer = buf.trim();
                        if (trimed_answer == "Y") | (trimed_answer == "y") {
                            println!("You input Y/y");
                            let data = "[credential]\n\thelper = store\n";
                            let file_path = Path::new("./test/.gitconfig_test");
                            match prepend_file(data.as_bytes(), &file_path) {
                                Ok(_) => println!("configured"),
                                _ => println!("failed to configured"),
                            }
                            println!("Please input your Github account");
                            print!("username: ");
                            let mut buf_username = String::new();
                            stdout().flush().unwrap();
                            stdin().read_line(&mut buf_username)?;
                            let trimed_username = buf_username.trim();
                            print!("password: ");
                            let mut buf_password = String::new();
                            stdout().flush().unwrap();
                            stdin().read_line(&mut buf_password)?;
                            let trimed_password = buf_password.trim();

                            let mut storage = fs::OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("./test/.git-credentials")
                                .expect("Can't find .g_credentials file");

                            match storage.write_all(
                                format!(
                                    "https://{}:{}@github.com",
                                    trimed_username, trimed_password
                                )
                                .as_bytes(),
                            ) {
                                Ok(()) => println!("done"),
                                Err(_) => println!("error"),
                            };
                        } else if (trimed_answer == "N") | (trimed_answer == "n") {
                            println!("You input N/n");
                        } else {
                            println!("You input wrong!");
                        }
                    } else if c.has_credential {
                        let stored_credentials = GCredential::new();

                        match stored_credentials {
                            Ok(sc) => {
                                let stdin = stdin();
                                let mut stdout = stdout().into_raw_mode().unwrap();

                                println!("{}", clear::All);

                                for index in 0..sc.credentials.len() {
                                    if let Some(cred) = sc.credentials.get(&index) {
                                        println!(
                                            "{} {}: {:?}",
                                            cursor::Goto(5, (2 + index).try_into().unwrap_or(2)),
                                            index,
                                            cred
                                        );
                                    }
                                }

                                println!("{}", cursor::Hide);

                                let mut index = 0;

                                println!("{}-", cursor::Goto(4,(2+index).try_into().unwrap_or(2)));
                                for c in stdin.keys() {
                                    match c.unwrap() {
                                        Key::Char('q') => break,
                                        Key::Ctrl('c') => break,
                                        Key::Esc => break,
                                        Key::Up => {
                                            if index > 0{
                                                println!("{} ", cursor::Goto(4, (2+index).try_into().unwrap_or(2)));
                                                index -= 1;
                                            }else{
                                                println!("{} ", cursor::Goto(4, (2+index).try_into().unwrap_or(2)));
                                                index = sc.credentials.len() -1;
                                            }
                                            
                                        },
                                        Key::Down => {
                                            if index < sc.credentials.len()-1 {
                                                println!("{} ", cursor::Goto(4, (2+index).try_into().unwrap_or(2)));
                                                index +=1;
                                            }else{
                                                println!("{} ", cursor::Goto(4, (2+index).try_into().unwrap_or(2)));
                                                index = 0;
                                            }
                                        },
                                        _ => println!("other"),
                                    }
                                    println!("{}-", cursor::Goto(4,(2+index).try_into().unwrap_or(2)));
                                    stdout.flush().unwrap();
                                }
                            }
                            Err(_) => {}
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
struct GCredential {
    credentials: HashMap<usize, (String, String)>,
}

impl GCredential {
    fn new() -> Result<GCredential, Box<dyn Error + Send + Sync>> {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./test/.g_credentials")
            .expect("Can't open .g_credentials file");

        let mut stored_credentials = String::new();
        f.read_to_string(&mut stored_credentials);

        let mut credentials: HashMap<usize, (String, String)> = HashMap::new();
        for (count, line) in stored_credentials.lines().enumerate() {
            let trimmed_line = line.trim();
            let splitted = trimmed_line.split_whitespace().collect::<Vec<&str>>();
            credentials.insert(count, (splitted[0].to_string(), splitted[1].to_string()));
        }

        Ok(GCredential { credentials })
    }
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

        for (_count, line) in gitconfig.lines().enumerate() {
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

fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> io::Result<()> {
    let tmp_path = Temp::new_file()?;
    let mut tmp = fs::File::create(&tmp_path)?;
    let mut src = fs::File::open(&file_path)?;
    tmp.write_all(&data)?;
    copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::rename(&tmp_path, &file_path)?;
    tmp_path.release();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
