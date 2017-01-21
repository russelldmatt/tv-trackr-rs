#![feature(conservative_impl_trait)]
#![feature(proc_macro)]
#![feature(field_init_shorthand)]
#![feature(try_from)]
#![feature(ordering_chaining)]

extern crate iron;
extern crate persistent;
extern crate handlebars;
extern crate handlebars_iron;
#[macro_use] extern crate router;
extern crate mount;
extern crate staticfile;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate bincode;
extern crate time;
#[macro_use] extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate itertools;

use iron::prelude::*;

pub mod show;
pub mod scraped_show;
pub mod viewer_history;
mod unique_id;
pub mod hello_world;
pub mod log_file;
pub mod counter;
pub mod hi;
pub mod template;

use iron::middleware::Handler;
use persistent::{State};

#[derive(Clone, Copy)]
pub enum Update {
    Seen,
    NotSeen,
}

pub fn seen_show_handler(update: Update, up_to: bool) -> impl Handler {

    fn seen_show(update: Update, up_to: bool, request: &mut Request) -> IronResult<Response> {
        use iron::status;    
        let mut payload = String::new();
        use std::io::Read;
        request.body.read_to_string(&mut payload).unwrap();
        println!("payload: {}", payload);
        use std::str::FromStr;
        let unique_id = 
            show::UniqueId::from_str(&payload)
            .expect("Could not parse unique_id from payload");
        println!("unique id: {:?}", unique_id);

        let unique_ids_to_check = match up_to {
            false => vec![unique_id],
            true => {
                let shows = request.get::<persistent::Read<show::Shows>>().unwrap();
                let shows = shows.as_ref();
                let show_name = unique_id.show.clone();
                match shows.iter().find(|&show| { show.name == show_name }) {
                    None => vec![],
                    Some(show) => {
                        show.episodes.keys().filter(|uid : &&show::UniqueId| {
                            *uid <= &unique_id
                        }).map(|uid : &show::UniqueId| (*uid).clone()).collect()
                    }
                }
            }
        };

        let arc = request.get_mut::<State<viewer_history::ViewerHistory>>().unwrap();
        let seen_shows = arc.as_ref();

        let newly_updated = | unique_id: show::UniqueId | -> bool { 
            let should_update = {
                let seen_shows = seen_shows.read().unwrap();
                let seen_already = seen_shows.contains(&unique_id);
                let updated = match update {
                    Update::Seen => !seen_already,
                    Update::NotSeen => seen_already,
                };
                println!("Should this unique id be updated? {}", updated);
                updated
            };
            if should_update {
                println!("Updating now");
                let mut seen_shows = seen_shows.write().unwrap();
                println!("Got lock");
                match update { 
                    Update::Seen => {
                        seen_shows
                        .insert_and_append(unique_id.clone())
                        .expect("Could not save seen shows")
                    },
                    Update::NotSeen => {
                        seen_shows.remove(&unique_id);
                        seen_shows
                            .save()
                            .expect("Could not save seen shows")
                    },
                }
            };
            should_update
        };
        // let request: Greeting = json::decode(&payload).unwrap();
        // let greeting = Greeting { msg: request.msg };
        // let payload = json::encode(&greeting).unwrap();

        use serde_json;
        let updates = unique_ids_to_check.iter().filter(|&unique_id| {
            newly_updated(unique_id.clone())
        });
        let json_response = {
            serde_json::value::Value::Array(
                updates.map(|unique_id: &show::UniqueId| {
                    serde_json::value::Value::String(unique_id.to_string())
                }).collect()
            )
        };
        let response = format!("{}", json_response);
        Ok(Response::with((status::Ok, response)))
    }

    Chain::new(move |request: &mut Request| { seen_show(update, up_to, request) })
}

#[cfg(test)]
mod tests { 
    #[test]
    fn chrono_to_string() {
        use chrono::*;
        let now = Local::today();
        println!("{}", now);
        let s = format!("{}", now.format("%B %d, %Y"));
        println!("{}", s);
        assert!(false);
    }

    #[test]
    fn chrono_of_string() {
        use chrono::*;
        let date = "December 23, 2016";
        let mut parsed = format::parsed::Parsed::new();
        format::parse(&mut parsed, date, format::strftime::StrftimeItems::new("%B %d, %Y")).unwrap();
        // let now = Local::today();
        // println!("{}", now);
        // let s = format!("{}", now.format("%B %d, %Y"));
        println!("{:?}", parsed);
        println!("{}", parsed.to_naive_date().unwrap());
        assert!(false);
    }

    #[test]
    fn module_stuff() {
        use unique_id;
        use std::str::FromStr;
        let _uid = unique_id::UniqueId::from_str("show-name.6.2");
        use show;
        let _uid = show::UniqueId::from_str("show-name.6.2");
        assert!(true)
    }
}


