use show;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct UniqueId {
    pub show: show::Name,
    pub season: i32,
    pub episode: i32,
}

impl ToString for UniqueId {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.show, self.season, self.episode)
    }
}

use std::num::ParseIntError;
#[derive(Debug)]
pub enum ParseUniqueIdError {
    NoEpisode,
    CannotParseEpisode(ParseIntError),
    NoSeason,
    CannotParseSeason(ParseIntError),
    NoShow,
}

impl FromStr for UniqueId {
    type Err = ParseUniqueIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ParseUniqueIdError::*;
        let mut rsplit = s.rsplit(".");
        let episode: i32 =
            rsplit.next().ok_or(NoEpisode)?.parse().map_err(CannotParseEpisode)?;
        let season: i32 =
            rsplit.next().ok_or(NoSeason)?.parse().map_err(CannotParseSeason)?;
        let show_parts: Vec<&str> = rsplit.collect();
        if show_parts.len() <= 0 {
            Err(NoShow)
        } else {
            use itertools::Itertools;
            let show = show::Name(show_parts.into_iter().rev().intersperse(".").collect());
            Ok(UniqueId { show, season, episode })
        }
    }
}

