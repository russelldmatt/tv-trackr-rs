#![feature(try_from)]

#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate mount;
extern crate staticfile;
extern crate tv_trackr;

use iron::prelude::*;
use tv_trackr::*;

fn main() {
    // CR mrussell: obviously change...
    let show_files = [
        "ballers.json",
        "broad-city.json",
        "brooklyn-99.json",
        "game-of-thrones.json",
        "its-always-sunny-in-philadelphia.json",
        "last-man-on-earth.json",
        "modern-family.json",
        "narcos.json",
        "south-park.json",
        "the-league.json",
        "the-night-of.json",
        "westworld.json",
        ];

    let shows: Vec<show::Show> = show_files.iter().map(|basename| {
        let file = format!("/Users/mrussell/code/tv-trackr/show-episodes/{}", basename);
        use scraped_show;
        let scraped_show: scraped_show::S = 
            scraped_show::load(&file)
            .map_err(|e| format!("Could not load scraped show {}, err: {}", file, e))
            .unwrap();
        use std::convert::TryFrom;
        show::Show::try_from(scraped_show).unwrap()
    }).collect();

    let seen_shows =
        viewer_history::load("/Users/mrussell/code/rust/tv-trackr/data/seen_shows.txt")
        .expect("Could not load seen shows");
    println!("#seen shows: {}", seen_shows.len());

    let router = router!(
        hello_world:  get "/"         => hello_world::handler(),
        log_file:     get "/log-file" => log_file::handler(),
        count:        get "/count"    => counter::handler(),
        hi:           get "/hi/:name" => hi::handler(),
        template:     get "/template" => template::handler(shows),
        seen_show:    post "/seen-show" => seen_show_handler(),
    );

    use mount::Mount;
    use staticfile::Static;
    use std::path::Path;
    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("/Users/mrussell/code/rust/tv-trackr/static")))
        ;

    // CR mrussell: It's not right that tv_trackr library code relies on this being done.
    let mut chain = Chain::new(mount);
    use persistent::{State};
    chain.link(State::<viewer_history::ViewerHistory>::both(seen_shows));
    
    let sock_addr = "localhost:3000";
    let _server = Iron::new(chain).http(sock_addr).unwrap();
    println!("serving on {}...", sock_addr);
}

