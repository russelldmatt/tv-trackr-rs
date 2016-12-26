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
        let uid = UniqueId { show: Name("hi.bye-name".to_string()), season: 4, episode: 10 };
        println!("{}", uid.to_string());
        let uid = UniqueId { show: Name("Modern Family".to_string()), season: 4, episode: 10 };
        let uid_str = uid.to_string();
        println!("{}", uid_str);
        use std::str::FromStr;
        let uid_round_tripped = UniqueId::from_str(&uid_str).unwrap();
        println!("eq? {}", uid_round_tripped == uid);
        assert!(false)
    }
}
