use chrono::naive::date::NaiveDate;
use std::str::FromStr;

#[derive(Debug)]
pub struct UniqueId {
    pub show: String,
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
            let show: String = show_parts.into_iter().rev().intersperse(".").collect();
            Ok(UniqueId { show, season, episode })
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    pub name: String,
    pub episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: i64,
    pub name: String,
    pub season: i32,
    pub episode: i32,
    pub aire_date: NaiveDate,
    pub seen_class: String,
}

#[cfg(test)]
mod tests { 
    use super::*;
    use chrono::naive::date::NaiveDate;

    fn today () -> NaiveDate {
        use chrono::offset::local::Local;
        use chrono::Datelike;
        let today = Local::today();
        NaiveDate::from_ymd(today.year(), today.month(), today.day())
    }

    lazy_static! {
        static ref EXAMPLE_EPISODE: Episode = 
            Episode {
                id: 1,
                name: "test".to_string(),
                season: 1,
                episode: 2,
                aire_date: today(),
                seen_class: "seen".to_string(),
            };
    }

    #[test]
    fn test_json() {
        use serde_json::value::ToJson;
        let json = Episode::to_json(&EXAMPLE_EPISODE);
        println!("{:?}", json);
        assert!(false)
    }

    #[test]
    fn unique_id_from_string() {
        use std::str::FromStr;
        println!("{:?}", UniqueId::from_str("show-name.6.2"));
        println!("{:?}", UniqueId::from_str("show.name.6.2"));
        println!("{:?}", UniqueId::from_str("Modern Family.6.2"));
        assert!(false)
    }

    #[test]
    fn unique_id_to_string() {
        let uid = UniqueId { show: "hi.bye-name".to_string(), season: 4, episode: 10 };
        println!("{}", uid.to_string());
        let uid = UniqueId { show: "Modern Family".to_string(), season: 4, episode: 10 };
        println!("{}", uid.to_string());
        assert!(false)
    }
}
