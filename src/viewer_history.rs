use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use show;

pub struct ShowsSeen(pub HashSet<show::UniqueId>);

pub fn load(file: &str) -> Result<ShowsSeen, io::Error> {
    let f = try!(File::open(file));
    let reader = BufReader::new(f);
    let shows: Result<HashSet<show::UniqueId>, io::Error> = {
        use std::str::FromStr;
        reader.lines().map(|line| {
            let line = try!(line);
            Ok(show::UniqueId::from_str(&line[..]).expect("Could not parse unique id"))
        }).collect()
    };
    let shows = try!(shows);
    Ok(ShowsSeen(shows))
}

