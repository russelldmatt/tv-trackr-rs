use chrono::naive::date::NaiveDate;

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
    use time;

    fn today () -> NaiveDate {
        use chrono::offset::local::Local;
        let today = Local::today();
        NaiveDate.from_ymd(today.year(), today.month(), today.date())
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
}
