use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use router::{Router};

pub fn handler() -> impl Handler {
    fn hi(req: &mut Request) -> IronResult<Response> {
        let params = req.extensions.get::<Router>().unwrap();
        let response = match params.find("name") {
            None => format!("Couldn't find name in params"),
            Some(name) => format!("Hi {}", name),
        };
        println!("{}", response);
        Ok(Response::with((status::Ok, response)))
    }
    Chain::new(hi)
}

