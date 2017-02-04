use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::sync;
use show;

#[derive(Clone, Copy)]
pub enum Update {
    Seen,
    NotSeen,
}

// Just for testing, so that I can control whether changes are saved to disk
enum CanSave<T> {
    Yes(T),
    No,
}

pub struct ShowsSeen {
    file: CanSave<String>,
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
        // CR mrussell: fix
        let file = if false { CanSave::Yes(file.to_string()) } else { CanSave::No };
        Ok(ShowsSeen { file, shows })
    }

    pub fn remove(&mut self, unique_id: &show::UniqueId) -> bool {
        self.shows.remove(unique_id)
    }

    /// Will create the file if it does not exists and overwrite if it does
    pub fn save(&self) -> io::Result<()> {
        match self.file {
            CanSave::No => Ok(()),
            CanSave::Yes(ref file) => {
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

    /// Will mutate self and save results.  It's more efficient to save incremental inserts rather than save the entire file.
    pub fn insert_and_save(&mut self, unique_id: show::UniqueId)
                           -> io::Result<()> {
        let new = self.shows.insert(unique_id.clone());
        if new {
            match self.file {
                CanSave::No => Ok(()),
                CanSave::Yes(ref file) => {
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

    /// Return value indicates whether this update changed the state of Self
    pub fn process_update(rwlocked_self: &sync::RwLock<Self>,
                          update: Update,
                          unique_id: show::UniqueId) 
                            -> bool {
        let should_update = {
            let seen_shows = rwlocked_self.read().unwrap();
            let seen_already = seen_shows.contains(&unique_id);
            let should_update = match update {
                Update::Seen => !seen_already,
                Update::NotSeen => seen_already,
            };
            println!("Should this unique id be updated? {}", should_update);
            should_update
        };
        if should_update {
            println!("Updating now");
            let mut seen_shows = rwlocked_self.write().unwrap();
            println!("Got lock");
            match update {
                Update::Seen => {
                    seen_shows
                        .insert_and_save(unique_id.clone())
                        .expect("Could not save seen shows")
                },
                // CR mrussell: Inefficient to do this for every
                // unique_id rather than once at the end.
                Update::NotSeen => {
                    seen_shows.remove(&unique_id);
                    seen_shows
                        .save()
                        .expect("Could not save seen shows")
                },
            }
        };
        should_update
    }
}

use iron::typemap::Key;
pub struct ViewerHistory;
impl Key for ViewerHistory { type Value = ShowsSeen; }

