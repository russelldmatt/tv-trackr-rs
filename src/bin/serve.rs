#![feature(try_from)]

#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate mount;
extern crate staticfile;
extern crate tv_trackr;

use iron::prelude::*;
use tv_trackr::*;
use viewer_history::Update;
use std::path::Path;

fn show_files(show_episode_dir: &str) -> Vec<String> {
    std::fs::read_dir(Path::new(show_episode_dir)).unwrap()
        .filter_map(|dir_entry| {
            let path_buf = dir_entry.unwrap().path();
            let path = path_buf.as_path();
            if path.is_dir() { 
                None
            } else {
                Some(path.to_str().unwrap().to_string())
            }
        })
        .collect()
}

fn shows(show_files: Vec<String>) -> Vec<show::Show> {
    use std::convert::TryFrom;
    show_files.iter().map(|file| {
        let scraped_show: scraped_show::Show = 
            scraped_show::load(&file)
            .map_err(|e| format!("Could not load scraped show {}, err: {}", file, e))
            .unwrap();
        show::Show::try_from(scraped_show).unwrap()
    }).collect()
}

fn main() {
    // CR mrussell: configurable
    let show_episode_dir = "/Users/mrussell/code/tv-trackr/show-episodes";
    let template_dir = "/Users/mrussell/code/rust/tv-trackr/templates/";
    let seen_shows_file = "/Users/mrussell/code/rust/tv-trackr/data/seen_shows.txt";
    let static_dir = "/Users/mrussell/code/rust/tv-trackr/static";

    let show_files : Vec<String> = show_files(show_episode_dir);
    for s in &show_files { println!("show file: {}", s) };
    let shows: Vec<show::Show> = shows(show_files); 
    let seen_shows =
        viewer_history::ShowsSeen::load(seen_shows_file)
        .expect("Could not load seen shows");
    println!("#seen shows: {}", seen_shows.len());

    let router = router!(
        template:                get  "/template"                => template::handler(&template_dir),
        seen_show:               post "/seen-show"               => update_handler(Update::Seen, false),
        seen_shows_up_to:        post "/seen-shows-up-to"        => update_handler(Update::Seen, true),
        havent_seen_show:        post "/havent-seen-show"        => update_handler(Update::NotSeen, false),
        havent_seen_shows_up_to: post "/havent-seen-shows-up-to" => update_handler(Update::NotSeen, true),
    );

    use mount::Mount;
    use staticfile::Static;
    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static", Static::new(Path::new(static_dir)))
        ;

    // CR mrussell: It's not right that tv_trackr library code relies on this being done.
    let mut chain = Chain::new(mount);
    use persistent::{State, Read};
    chain.link(State::<viewer_history::ViewerHistory>::both(seen_shows));
    chain.link_before(Read::<show::Shows>::one(shows));

    let sock_addr = "localhost:3000";
    let _server = Iron::new(chain).http(sock_addr).unwrap();
    println!("serving on {}...", sock_addr);
}

