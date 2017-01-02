#![feature(field_init_shorthand)]
extern crate iron;

use std::process::Command;
use std::env;
use iron::Url;

#[derive(Debug)]
pub enum CommandError {
    IoError(std::io::Error),
    ExitStatusNonZero { exit_status: Option<i32>, stderr: String },
    UnexpectedStdout(String),
    CouldNotFindShow,
    CannotParseUrl { url: String, error: String },
}

pub type Result<T> = std::result::Result<T, CommandError>;

pub fn tv_guide_url(search_show_name: String) -> Result<Url> {
    let mut command = Command::new("/Users/mrussell/code/rust/tv-trackr/get-tv-guide-url.sh");
    command.arg(search_show_name);
    let output = command.output().map_err(CommandError::IoError)?;
    if !output.status.success() {
        Err(CommandError::ExitStatusNonZero { 
            exit_status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let mut stdout_lines = stdout.lines();
        match stdout_lines.next() {
            None => {
                Err(CommandError::UnexpectedStdout(stdout.clone()))
            },
            Some(url) => {
                match url {
                    "null" => Err(CommandError::CouldNotFindShow),
                    url => {
                        let url = url.trim_matches('"');
                        Url::parse(url).map_err(|error| CommandError::CannotParseUrl { url: url.to_string(), error })
                    }
                }
            }
        }
    }
}

fn main() {
    let search = env::args().nth(1).expect("Expected an argument (tv show)");
    println!("url: {:?}", tv_guide_url(search));
}
