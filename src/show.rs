use time_wrapper::Time as Time;

#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    pub name: String,
    pub episodes: Vec<Episode>,
}

// CR mrussell: fix
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        // self.date = datetime.datetime.strptime(d['aire_date'], '%B %d, %Y').date()
        format!("{} {} {}", self.year, self.month, self.day)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: i64,
    pub name: String,
    pub season: i32,
    pub episode: i32,
    pub aire_date: Time,
    pub aire_date_string: String,
    pub seen_class: String,
}


#[cfg(test)]
mod tests { 
    use super::*;
    use time_wrapper::Time as Time;
    use time;

    lazy_static! {
        static ref EXAMPLE_EPISODE: Episode = {
            let aire_date = Time(time::now());
            Episode {
                id: 1,
                name: "test".to_string(),
                season: 1,
                episode: 2,
                aire_date: aire_date.clone(),
                aire_date_string: aire_date.strftime("%B %d, %Y").unwrap().to_string(),
                seen_class: "seen".to_string(),
            }
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
