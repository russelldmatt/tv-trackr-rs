use chrono::naive::date::NaiveDate;
pub use unique_id::UniqueId;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Name(pub String);

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    pub name: String,
    pub episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
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
                name: "test".to_string(),
                season: 1,
                episode: 2,
                aire_date: today(),
                seen_class: "seen".to_string(),
            };
    }

    #[test]
    fn test_json() {
        use serde_json;
        use serde_json::value::{ToJson, Value};
        let json = Episode::to_json(&EXAMPLE_EPISODE);
        let test_against: Value = 
            serde_json::from_str(r#"{"aire_date":"2016-12-26","episode":2,"name":"test","season":1,"seen_class":"seen"}"#).unwrap();
        assert_eq!(test_against, json);
    }
}
