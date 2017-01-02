use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use show;

// Just for testing, so that I can control whether changes are saved to disk
enum CanAlter<T> {
    Yes(T),
    No(T),
}

pub struct ShowsSeen {
    file: CanAlter<String>,
    pub shows: HashSet<show::UniqueId>,
}

use std::ops::Deref;
impl Deref for ShowsSeen {
    type Target = HashSet<show::UniqueId>;
    fn deref(&self) -> &Self::Target {
        &self.shows
    }
}

impl ShowsSeen {
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
        Ok(ShowsSeen { file: CanAlter::No(file.to_string()), shows })
    }

    pub fn save(&self) -> io::Result<()> {
        // File::create will create the file if it does not exists and truncate if it does
        match self.file {
            CanAlter::No(_) => Ok(()),
            CanAlter::Yes(ref file) => {
                let mut f = try!(File::create(file)); 
                self.iter().fold(Ok(()), |acc, unique_id| {
                    acc.and_then(|()| {
                        f.write_all(&unique_id.to_string().into_bytes())
                            .and_then(|()| f.write_all(b"\n"))
                    })
                })
            }
        }
    }

    pub fn insert_and_append(&mut self, unique_id: show::UniqueId)
                           -> io::Result<()> {
        let new = self.shows.insert(unique_id.clone());
        if new {
            match self.file {
                CanAlter::No(_) => Ok(()),
                CanAlter::Yes(ref file) => {
                    use std::fs::OpenOptions;
                    let mut f = try!(OpenOptions::new().append(true).open(file));
                    try!(f.write_all(&unique_id.to_string().into_bytes()));
                    f.write_all(b"\n")
                }
            }
        } else {
            Ok(())
        }
    }
}

use iron::typemap::Key;
pub struct ViewerHistory;
impl Key for ViewerHistory { type Value = ShowsSeen; }

