#![feature(conservative_impl_trait)]
#![feature(proc_macro)]
#![feature(field_init_shorthand)]

extern crate iron;
extern crate persistent;
extern crate handlebars;
extern crate handlebars_iron;
#[macro_use] extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate bincode;
extern crate time;
#[macro_use] extern crate chrono;
#[macro_use] extern crate lazy_static;

use iron::prelude::*;

mod time_wrapper;
mod show;
mod hello_world;
mod log_file;
mod counter;
mod hi;
mod template;

use iron::middleware::Handler;
fn test_post_handler() -> impl Handler {
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

        use rustc_serialize::json::Json;
        use rustc_serialize::json;
        let response = json::encode(&Json::String(format!("successfully received #{}", *count))).unwrap();
        Ok(Response::with((status::Ok, response)))
    }

    let mut chain = Chain::new(test_post);
    chain.link(State::<Counter>::both(0));
    chain
}

fn main() {
    let router = router!(
        hello_world:  get "/"         => hello_world::handler(),
        log_file:     get "/log-file" => log_file::handler(),
        count:        get  "/count"   => counter::handler(),
        hi:           get "/hi/:name" => hi::handler(),
        template:     get "/template" => template::handler(),
        test_post:    post "/test-post" => test_post_handler(),
    );

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;
    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("/Users/mrussell/code/rust/tv-trackr/static")))
        ;

    let sock_addr = "localhost:3000";
    let _server = Iron::new(mount).http(sock_addr).unwrap();
    println!("serving on {}...", sock_addr);
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
}


