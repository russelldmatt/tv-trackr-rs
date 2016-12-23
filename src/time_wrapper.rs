use time;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqVisitor, MapVisitor, Error};

// Wrapping time so that I can implement traits for it
#[derive(Clone, Debug, PartialEq)]
pub struct Time(pub time::Tm);

use std::ops::Deref;
impl Deref for Time {
    type Target = time::Tm;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Time", 11)?;
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

impl Deserialize for Time {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        enum Field {
            TmSec,
            TmMin,
            TmHour,
            TmMday,
            TmMon,
            TmYear,
            TmWday,
            TmYday,
            TmIsdst,
            TmUtcoff,
            TmNsec,
        };

        impl Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: Deserializer,
            {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: Error,
                    {
                        match value {
                            "tm_sec" => Ok(Field::TmSec),
                            "tm_min" => Ok(Field::TmMin),
                            "tm_hour" => Ok(Field::TmHour),
                            "tm_mday" => Ok(Field::TmMday),
                            "tm_mon" => Ok(Field::TmMon),
                            "tm_year" => Ok(Field::TmYear),
                            "tm_wday" => Ok(Field::TmWday),
                            "tm_yday" => Ok(Field::TmYday),
                            "tm_isdst" => Ok(Field::TmIsdst),
                            "tm_utcoff" => Ok(Field::TmUtcoff),
                            "tm_nsec" => Ok(Field::TmNsec),
                            _ => Err(Error::unknown_field(value)),
                        }
                    }
                }
                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct TimeVisitor;

        impl Visitor for TimeVisitor {
            type Value = Time;

            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Time, V::Error>
                where V: SeqVisitor,
            {
                let try = |visitor: &mut V, len| {
                    let value: Option<i32> = try!(visitor.visit());
                    match value {
                        Some(value) => Ok(value),
                        None => {
                            try!(visitor.end());
                            Err(Error::invalid_length(len))
                        }
                }};
                let tm_sec = try(&mut visitor, 0)?;
                let tm_min = try(&mut visitor, 1)?;
                let tm_hour = try(&mut visitor, 2)?;
                let tm_mday = try(&mut visitor, 3)?;
                let tm_mon = try(&mut visitor, 4)?;
                let tm_year = try(&mut visitor, 5)?;
                let tm_wday = try(&mut visitor, 6)?;
                let tm_yday = try(&mut visitor, 7)?;
                let tm_isdst = try(&mut visitor, 8)?;
                let tm_utcoff = try(&mut visitor, 9)?;
                let tm_nsec = try(&mut visitor, 10)?;
                try!(visitor.end());
                let time = 
                    time::Tm {
                        tm_sec,
                        tm_min,
                        tm_hour,
                        tm_mday,
                        tm_mon,
                        tm_year,
                        tm_wday,
                        tm_yday,
                        tm_isdst,
                        tm_utcoff,
                        tm_nsec,
                    };
                Ok(Time(time))
            }

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Time, V::Error>
                where V: MapVisitor,
            {
                let mut tm_sec: Option<i32> = None;
                let mut tm_min: Option<i32> = None;
                let mut tm_hour: Option<i32> = None;
                let mut tm_mday: Option<i32> = None;
                let mut tm_mon: Option<i32> = None;
                let mut tm_year: Option<i32> = None;
                let mut tm_wday: Option<i32> = None;
                let mut tm_yday: Option<i32> = None;
                let mut tm_isdst: Option<i32> = None;
                let mut tm_utcoff: Option<i32> = None;
                let mut tm_nsec: Option<i32> = None;
                while let Some(key) = try!(visitor.visit_key::<Field>()) {
                    match key {
                        Field::TmSec => {
                            if tm_sec.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_sec"));
                            }
                            tm_sec = Some(try!(visitor.visit_value()));
                        }
                        Field::TmMin => {
                            if tm_min.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_min"));
                            }
                            tm_min = Some(try!(visitor.visit_value()));
                        }
                        Field::TmHour => {
                            if tm_hour.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_hour"));
                            }
                            tm_hour = Some(try!(visitor.visit_value()));
                        }
                        Field::TmMday => {
                            if tm_mday.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_mday"));
                            }
                            tm_mday = Some(try!(visitor.visit_value()));
                        }
                        Field::TmMon => {
                            if tm_mon.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_mon"));
                            }
                            tm_mon = Some(try!(visitor.visit_value()));
                        }
                        Field::TmYear => {
                            if tm_year.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_year"));
                            }
                            tm_year = Some(try!(visitor.visit_value()));
                        }
                        Field::TmWday => {
                            if tm_wday.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_wday"));
                            }
                            tm_wday = Some(try!(visitor.visit_value()));
                        }
                        Field::TmYday => {
                            if tm_yday.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_yday"));
                            }
                            tm_yday = Some(try!(visitor.visit_value()));
                        }
                        Field::TmIsdst => {
                            if tm_isdst.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_isdst"));
                            }
                            tm_isdst = Some(try!(visitor.visit_value()));
                        }
                        Field::TmUtcoff => {
                            if tm_utcoff.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_utcoff"));
                            }
                            tm_utcoff = Some(try!(visitor.visit_value()));
                        }
                        Field::TmNsec => {
                            if tm_nsec.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("tm_nsec"));
                            }
                            tm_nsec = Some(try!(visitor.visit_value()));
                        }
                    }
                }
                try!(visitor.end());
                let field_value = |optional_value: Option<i32>, name| {
                    match optional_value {
                        Some(value) => Ok(value),
                        None => Err(<V::Error as Error>::missing_field(name)),
                    }
                };
                let time = 
                    time::Tm {
                        tm_sec: field_value(tm_sec, "tm_sec")?,
                        tm_min: field_value(tm_min, "tm_min")?,
                        tm_hour: field_value(tm_hour, "tm_hour")?,
                        tm_mday: field_value(tm_mday, "tm_mday")?,
                        tm_mon: field_value(tm_mon, "tm_mon")?,
                        tm_year: field_value(tm_year, "tm_year")?,
                        tm_wday: field_value(tm_wday, "tm_wday")?,
                        tm_yday: field_value(tm_yday, "tm_yday")?,
                        tm_isdst: field_value(tm_isdst, "tm_isdst")?,
                        tm_utcoff: field_value(tm_utcoff, "tm_utcoff")?,
                        tm_nsec: field_value(tm_nsec, "tm_nsec")?,
                    };
                Ok(Time(time))
            }
        }

        const FIELDS: &'static [&'static str] =
            &[
                "tm_sec",
                "tm_min",
                "tm_hour",
                "tm_mday",
                "tm_mon",
                "tm_year",
                "tm_wday",
                "tm_yday",
                "tm_isdst",
                "tm_utcoff",
                "tm_nsec",
            ];
        deserializer.deserialize_struct("Time", FIELDS, TimeVisitor)
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    use time;

    #[test]
    fn test() {
        let time = time::strptime("June 28, 2015", "%B %d, %Y").unwrap();
        println!("time: {:?}", time);
        println!("{}", time.strftime("%Y-%m-%d").unwrap());
        println!("{}", time.strftime("%B %d, %Y").unwrap());
        assert!(false)
    }

    #[test]
    fn test2() {
        use serde_json;
        use serde::Serialize;
        let time = Time(time::strptime("June 28, 2015", "%B %d, %Y").unwrap());
        println!("time: {:?}", time);
        let mut json_serializer = serde_json::value::Serializer::new();
        println!("json: {:?}", time.serialize(&mut json_serializer));
        println!("json_serializer: {}", json_serializer.unwrap());
        let json = serde_json::value::to_value(time.clone());
        println!("json: {}", json);
        let round_tripped: Time = serde_json::value::from_value(json).unwrap();
        assert!(round_tripped == time);
        assert!(false);
    }

    #[test]
    fn test3() {
        use bincode;
        let time = Time(time::strptime("June 28, 2015", "%B %d, %Y").unwrap());
        println!("time: {:?}", time);
        let bytes = bincode::serde::serialize(&time, bincode::SizeLimit::Infinite).unwrap();
        println!("bytes: {:?}", bytes);
        let round_tripped: Time = bincode::serde::deserialize(&bytes[..]).unwrap();
        println!("round_tripped: {:?}", round_tripped);
        println!("eq?: {}", round_tripped == time);
        assert!(round_tripped == time);
        assert!(false);
    }
}
