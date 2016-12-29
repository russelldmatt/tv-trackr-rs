#![feature(try_from)]
extern crate tv_trackr;

use std::env;
use tv_trackr::*;

fn main() {
    let file = env::args().nth(1).unwrap();
    println!("loading file: {}", file);
    let scraped_show = scraped_show::load(&file);
    println!("{:?}", scraped_show);
    use std::convert::TryFrom;
    println!("{:?}", show::Show::try_from(scraped_show.unwrap()));
}
