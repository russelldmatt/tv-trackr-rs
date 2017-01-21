use chrono::naive::date::NaiveDate;
use std::fmt;
use std::collections::HashMap;
pub use unique_id::UniqueId; 

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
pub struct Name(pub String);

use std::ops::Deref;
impl Deref for Name {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Name {
    fn from(s: &str) -> Name {
        Name (s.to_string())
    }
}

use std::cmp::Ordering;
impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Show {
    pub name: Name,
    pub episodes: HashMap<UniqueId, Episode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    pub name: String,
    pub season: i32,
    pub episode: i32,
    pub aire_date: NaiveDate,
}

impl Episode {
    pub fn unique_id(&self, show: Name) -> UniqueId {
        UniqueId { show, season: self.season, episode: self.episode }
    }
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
                name: "test".to_string(),
                season: 1,
                episode: 2,
                aire_date: today(),
            };
    }

    #[test]
    fn test_json() {
        use serde_json;
        use serde_json::value::{ToJson, Value};
        let json = Episode::to_json(&EXAMPLE_EPISODE);
        let test_against: Value = 
            serde_json::from_str(r#"{"aire_date":"2016-12-26","episode":2,"name":"test","season":1}"#).unwrap();
        assert_eq!(test_against, json);
    }
}

use iron::typemap::Key;
#[derive(Copy, Clone)]
pub struct Shows;
impl Key for Shows { type Value = Vec<Show>; }

