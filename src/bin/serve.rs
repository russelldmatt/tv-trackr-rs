#[macro_use] extern crate router;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate tv_trackr;

use iron::prelude::*;
use tv_trackr::*;

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

