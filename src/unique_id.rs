use show;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize, Clone)]
pub struct UniqueId {
    pub show: show::Name,
    pub season: i32,
    pub episode: i32,
}

use std::cmp::Ordering;
impl Ord for UniqueId {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then(self.show.cmp(&other.show))
            .then(self.season.cmp(&other.season))
            .then(self.episode.cmp(&other.episode))
    }
}
impl UniqueId {
    fn delim() -> char {
        '_'
    }
}

impl ToString for UniqueId {
    fn to_string(&self) -> String {
        format!("{}{}{}{}{}", self.show, Self::delim(), self.season, Self::delim(), self.episode)
    }
}

use std::num::ParseIntError;
#[derive(Debug, PartialEq)]
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
        let mut rsplit = s.rsplit(Self::delim());
        let episode: i32 =
            rsplit.next().ok_or(NoEpisode)?.parse().map_err(CannotParseEpisode)?;
        let season: i32 =
            rsplit.next().ok_or(NoSeason)?.parse().map_err(CannotParseSeason)?;
        let show_parts: Vec<&str> = rsplit.collect();
        if show_parts.len() <= 0 {
            Err(NoShow)
        } else {
            use itertools::Itertools;
            let show = show::Name(show_parts.into_iter().rev()
                                  .intersperse(&Self::delim().to_string())
                                  .collect()
            );
            Ok(UniqueId { show, season, episode })
        }
    }
}

#[cfg(test)]
mod tests { 
    use show::*;

    #[test]
    fn unique_id_from_string() {
        use std::str::FromStr;
        assert_eq!(Ok (UniqueId { show: Name::from("show-name"), season: 6, episode: 2 }),
                   UniqueId::from_str("show-name.6.2")
        );
        assert_eq!(Ok (UniqueId { show: Name::from("show.name"), season: 6, episode: 2 }),
                   UniqueId::from_str("show.name.6.2")
        );
        assert_eq!(Ok (UniqueId { show: Name::from("Modern Family"), season: 6, episode: 2 }),
                   UniqueId::from_str("Modern Family.6.2")
        );
    }

    #[test]
    fn unique_id_to_string() {
        let uid = UniqueId { show: Name::from("hi.bye-name"), season: 4, episode: 10 };
        assert_eq!("hi.bye-name.4.10", uid.to_string());
        let uid = UniqueId { show: Name::from("Modern Family"), season: 4, episode: 10 };
        assert_eq!("Modern Family.4.10", uid.to_string());
    }

    #[test]
    fn round_trip() {
        use std::str::FromStr;
        let uid = UniqueId { show: Name::from("Modern Family"), season: 4, episode: 10 };
        let uid_str = uid.to_string();
        let uid_round_tripped = UniqueId::from_str(&uid_str).unwrap();
        assert_eq!(uid_round_tripped, uid)
    }
}
