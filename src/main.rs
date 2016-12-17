#![feature(conservative_impl_trait)]
#![feature(proc_macro)]

extern crate iron;
extern crate persistent;
extern crate handlebars_iron;
#[macro_use] extern crate router;
extern crate mount;
extern crate staticfile;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;
extern crate chrono;

use iron::prelude::*;

mod hello_world;
mod log_file;
mod counter;
mod hi;
mod template;

fn main() {
    // use iron::Handler;
    // fn static_file_mount() -> impl Handler {
    //     println!("in static_file_mount");
    //     let mut mount = Mount::new();
    //     mount.mount("/", Static::new(Path::new("/Users/mrussell/code/rust/tv-trackr/target/debug/tv-trackr/static/")));
    //     mount
    // }

    let router = router!(
        hello_world:  get "/"         => hello_world::handler(),
        log_file:     get "/log-file" => log_file::handler(),
        count:        get  "/count"   => counter::handler(),
        hi:           get "/hi/:name" => hi::handler(),
        template:     get "/template" => template::handler(),
        // static_files: get "/static"   => static_file_mount(),
    );

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;
    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("/Users/mrussell/code/rust/tv-trackr/static")));

    let sock_addr = "localhost:3000";
    let _server = Iron::new(mount).http(sock_addr).unwrap();
    println!("serving on {}...", sock_addr);
}


