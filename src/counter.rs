use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::typemap::Key;

use persistent::{State};

#[derive(Copy, Clone)]
pub struct Counter;
impl Key for Counter { type Value = i32; }

pub fn handler() -> impl Handler {
    fn show_count(req: &mut Request) -> IronResult<Response> {
        println!("show count");
        let arc = req.get_mut::<State<Counter>>().unwrap();
        let count = arc.as_ref();
        let mut count = count.write().unwrap();
        *count += 1;
        Ok(Response::with((status::Ok, format!("Count: {:?}", *count))))
    }

    let mut chain = Chain::new(show_count);
    chain.link(State::<Counter>::both(0));
    chain
}

