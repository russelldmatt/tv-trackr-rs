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
mod viewer_history;
mod unique_id;
pub mod hello_world;
pub mod log_file;
pub mod counter;
pub mod hi;
pub mod template;

use iron::middleware::Handler;
pub fn test_post_handler() -> impl Handler {
    use iron::typemap::Key;
    use persistent::{State};

    #[derive(Copy, Clone)]
    pub struct Counter;
    impl Key for Counter { type Value = i32; }

    fn test_post(request: &mut Request) -> IronResult<Response> {
        use iron::status;    
        let mut payload = String::new();
        use std::io::Read;
        request.body.read_to_string(&mut payload).unwrap();
        println!("payload: {}", payload);
        // let request: Greeting = json::decode(&payload).unwrap();
        // let greeting = Greeting { msg: request.msg };
        // let payload = json::encode(&greeting).unwrap();

        let arc = request.get_mut::<State<Counter>>().unwrap();
        let count = arc.as_ref();
        let mut count = count.write().unwrap();
        *count += 1;

        use serde_json;
        let json_response = 
            serde_json::value::Value::String(format!("successfully received #{}", *count));
        let response = format!("{}", json_response);
        Ok(Response::with((status::Ok, response)))
    }

    let mut chain = Chain::new(test_post);
    chain.link(State::<Counter>::both(0));
    chain
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


