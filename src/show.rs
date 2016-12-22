use time;
use serde;

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

// Wrapping time so that I can implement traits for it
#[derive(Clone, Debug)]
pub struct Time(pub time::Tm);

impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        // pub struct Tm {
        //     pub tm_sec: i32,
        //     pub tm_min: i32,
        //     pub tm_hour: i32,
        //     pub tm_mday: i32,
        //     pub tm_mon: i32,
        //     pub tm_year: i32,
        //     pub tm_wday: i32,
        //     pub tm_yday: i32,
        //     pub tm_isdst: i32,
        //     pub tm_utcoff: i32,
        //     pub tm_nsec: i32,
        // }
        // CR mrussell: should len be 11 or 11 * 4?
        let mut state = serializer.serialize_struct("Tm", 11)?;
        serializer.serialize_struct_elt(&mut state, "tm_sec", self.tm_sec)?;
        serializer.serialize_struct_elt(&mut state, "tm_min", self.tm_min)?;
        serializer.serialize_struct_elt(&mut state, "tm_hour", self.tm_hour)?;
        serializer.serialize_struct_elt(&mut state, "tm_mday", self.tm_mday)?;
        serializer.serialize_struct_elt(&mut state, "tm_mon", self.tm_mon)?;
        serializer.serialize_struct_elt(&mut state, "tm_year", self.tm_year)?;
        serializer.serialize_struct_elt(&mut state, "tm_wday", self.tm_wday)?;
        serializer.serialize_struct_elt(&mut state, "tm_yday", self.tm_yday)?;
        serializer.serialize_struct_elt(&mut state, "tm_isdst", self.tm_isdst)?;
        serializer.serialize_struct_elt(&mut state, "tm_utcoff", self.tm_utcoff)?;
        serializer.serialize_struct_elt(&mut state, "tm_nsec", self.tm_nsec)?;
        serializer.serialize_struct_end(state)
    }
}

use std::ops::Deref;
impl Deref for Time {
    type Target = time::Tm;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// CR mrussell: fix
fn deserialize_tm<D>(_deserializer: &mut D) -> Result<Time, D::Error> 
    where D: serde::Deserializer {
    Ok(Time(time::now()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: i64,
    pub name: String,
    pub season: i32,
    pub episode: i32,
    #[serde(deserialize_with = "deserialize_tm")]
    pub aire_date: Time,
    pub aire_date_string: String,
    pub seen_class: String,
}

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
fn test() {
    let time = time::strptime("June 28, 2015", "%B %d, %Y").unwrap();
    println!("time: {:?}", time);
    println!("{}", time.strftime("%Y-%m-%d").unwrap());
    println!("{}", time.strftime("%B %d, %Y").unwrap());
    assert!(false)
}

#[test]
fn test_json() {
    use serde_json::value::ToJson;
    let json = Episode::to_json(&EXAMPLE_EPISODE);
    println!("{:?}", json);
    assert!(false)
}
