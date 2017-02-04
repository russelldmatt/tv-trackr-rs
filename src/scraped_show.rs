use std;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use show;

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    name: String,
    episode: String,
    aire_date: String,
}

use std::num::ParseIntError;    
#[derive(Debug, PartialEq)]
pub enum ParseError {
    CannotFindInString,
    CannotFindInt,
    CannotParseInt(ParseIntError),
}

#[derive(Debug, PartialEq)]
pub enum ParseEpisodeError {
    ParseSeasonError(ParseError),
    ParseEpisodeError(ParseError),
}

use std::convert::TryFrom;
impl TryFrom<Episode> for show::Episode {
    type Err = ParseEpisodeError;

    fn try_from(e: Episode) -> Result<Self, Self::Err> {
        let mut episode = e.episode.split(", ");
        use std::str::Split;
        let parse_season_or_episode = |name: &mut Split<&str>, prefix: &str| {
            let season_or_episode = 
                name.next()
                .ok_or(ParseError::CannotFindInString)?;
            let int_string = 
                season_or_episode.split(prefix).nth(1)
                .ok_or(ParseError::CannotFindInt)?;
            int_string.parse()
                .map_err(ParseError::CannotParseInt)
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
            show::Episode {
                name: e.name,
                season: season,
                episode: episode,
                aire_date,
            };
        Ok (episode)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    name: String,
    episodes: Vec<Episode>,
}

impl TryFrom<Show> for show::Show {
    type Err = ParseEpisodeError;

    fn try_from(s: Show) -> Result<Self, Self::Err> {
        let show_name = show::Name::from(&s.name[..]);
        let eps: Result<Vec<show::Episode>, Self::Err> = 
            s.episodes.into_iter().map(|e| show::Episode::try_from(e)).collect();
        let episodes: HashMap<show::UniqueId, show::Episode> = 
            eps?.into_iter().map(|ep| (ep.unique_id(show_name.clone()), ep)).collect();
        Ok (
            show::Show {
                name: show_name,
                episodes
            }
        )
    }
}

pub fn load(file: &str) -> Result<Show, std::io::Error> {
    use serde_json;

    use std::path::Path;
    let name = Path::new(file).file_stem().unwrap().to_str().unwrap().to_string();
    let f = try!(File::open(file));
    let reader = BufReader::new(f);

    let eps: Result<Vec<Episode>, std::io::Error> = reader.lines().map(|line| {
        let line: String = try!(line);
        Ok(serde_json::from_str(&line).unwrap())
    }).collect();
    
    let episodes = try!(eps);
    Ok (Show { name, episodes })
}

