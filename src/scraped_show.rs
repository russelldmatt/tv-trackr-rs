use std;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use show::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct E {
    name: String,
    episode: String,
    aire_date: String,
}

use std::num::ParseIntError;    
#[derive(Debug, PartialEq)]
pub enum ParseSeasonOrEpisodeError {
    CannotFindInEpisodeString,
    CannotFindInt,
    CannotParseInt(ParseIntError),
}

#[derive(Debug, PartialEq)]
pub enum ParseEpisodeError {
    ParseSeasonError(ParseSeasonOrEpisodeError),
    ParseEpisodeError(ParseSeasonOrEpisodeError),
}

use std::convert::TryFrom;
impl TryFrom<E> for Episode {
    type Err = ParseEpisodeError;

    fn try_from(e: E) -> Result<Self, Self::Err> {
        let mut episode = e.episode.split(", ");
        use std::str::Split;
        let parse_season_or_episode = |name: &mut Split<&str>, prefix: &str| {
            let season_or_episode = 
                name.next()
                .ok_or(ParseSeasonOrEpisodeError::CannotFindInEpisodeString)?;
            let int_string = 
                season_or_episode.split(prefix).nth(1)
                .ok_or(ParseSeasonOrEpisodeError::CannotFindInt)?;
            int_string.parse()
                .map_err(ParseSeasonOrEpisodeError::CannotParseInt)
        };
        let season: i32 = 
            parse_season_or_episode(&mut episode, "Season ")
            .map_err(ParseEpisodeError::ParseSeasonError)?;
        let episode: i32 = 
            parse_season_or_episode(&mut episode, "Episode ")
            .map_err(ParseEpisodeError::ParseEpisodeError)?;

        use chrono;
        let format_string = 
            chrono::format::strftime::StrftimeItems::new("%B %d, %Y");
        let mut parsed = chrono::format::parsed::Parsed::new();
        chrono::format::parse(&mut parsed, &e.aire_date, format_string).unwrap();
        let aire_date = parsed.to_naive_date().unwrap();
        
        let episode = 
            Episode {
                name: e.name,
                season: season,
                episode: episode,
                aire_date,
            };
        Ok (episode)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct S {
    name: String,
    episodes: Vec<E>,
}

impl TryFrom<S> for Show {
    type Err = ParseEpisodeError;

    fn try_from(s: S) -> Result<Self, Self::Err> {
        let show_name = Name::from(&s.name[..]);
        let eps: Result<Vec<Episode>, Self::Err> = 
            s.episodes.into_iter().map(|e| Episode::try_from(e)).collect();
        let episodes: HashMap<UniqueId, Episode> = 
            eps?.into_iter().map(|ep| (ep.unique_id(show_name.clone()), ep)).collect();
        Ok (
            Show {
                name: show_name,
                episodes
            }
        )
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn load(file: &String) -> Result<S, std::io::Error> {
    use serde_json;

    use std::path::Path;
    let name = Path::new(file).file_stem().unwrap().to_str().unwrap().to_string();
    use itertools::Itertools;
    let name = name.split("-").into_iter().map(capitalize)
        .intersperse(" ".to_string())
        .collect();
    let f = try!(File::open(file));
    let reader = BufReader::new(f);

    let eps: Result<Vec<E>, std::io::Error> = reader.lines().map(|line| {
        let line: String = try!(line);
        Ok(serde_json::from_str(&line).unwrap())
    }).collect();
    
    let episodes = eps?;
    Ok (S { name, episodes })
}

