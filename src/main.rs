#![feature(conservative_impl_trait)]

extern crate iron;
extern crate persistent;
#[macro_use] extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;

mod hello_world;
mod log_file;
mod counter;
mod hi;

fn main() {
    let router = router!(
        hello_world: get  "/"        => hello_world::handler(),
        log_file:    get "/log-file" => log_file::handler(),
        count:       get  "/count"   => counter::handler(),
        hi:          get "/hi/:name" => hi::handler(),
    );

    let sock_addr = "localhost:3000";
    let _server = Iron::new(router).http(sock_addr).unwrap();
    println!("serving on {}...", sock_addr);
}


