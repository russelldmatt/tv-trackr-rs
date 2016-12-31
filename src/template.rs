use time;
use std::time::Instant;
use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};
use serde_json;

use iron::typemap::Key;
use persistent::{Read};

use show;
use show::*;

#[derive(Copy, Clone)]
pub struct Shows;
impl Key for Shows { type Value = Vec<Show>; }

fn handle_shows(req: &mut Request) -> IronResult<Response> {
    println!("shows");
    let start = Instant::now();
    println!("elapsed at 1: {:?}", start.elapsed());

    let mut response = Response::new();
    let shows = req.get::<Read<Shows>>().unwrap();
    let shows = shows.as_ref();

    #[derive(Serialize, Debug)]    
    pub struct EpisodeWithId {
        unique_id: String,
        episode: Episode 
    }

    #[derive(Serialize, Debug)]
    pub struct Show {
        pub name: Name,
        pub episodes: Vec<EpisodeWithId>,
    }

    #[derive(Serialize, Debug)]
    struct Data { 
        shows: Vec<Show>,
    }

    let shows = shows.into_iter().map(|show: &show::Show| { 
        let mut episodes: Vec<(UniqueId, Episode)> =
            show.episodes.iter()
            .map(|(id, ep)| (id.clone(), ep.clone()))
            .collect();
        episodes.sort_by(|fst, snd| fst.0.cmp(&snd.0));
        episodes.reverse();
        let episodes = episodes.into_iter()
            .map(|(unique_id, episode)| 
                 EpisodeWithId { unique_id: unique_id.to_string(), 
                                 episode 
                 })
            .collect();
        Show { name: show.name.clone(), 
               episodes
        }
    }).collect();
    let data = Data { shows: shows };

    response.set_mut(Template::new("episodes", data)).set_mut(status::Ok);
    println!("{}", response);
    println!("elapsed at 2: {:?}", start.elapsed());
    Ok(response)
}

pub fn handler() -> impl Handler {
    // helpers
    // https://github.com/sunng87/handlebars-rust#extensible-helper-system
    use handlebars::{Helper, Handlebars, Context, RenderContext, RenderError};
    use chrono;
    fn date_helper (_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext)
                    -> Result<(), RenderError> {
        println!("{}", time::strftime("%Y-%m-%d: %H:%M:%S.%f", &time::now()).expect("Bad time format string"));
        // just for example, add error check for unwrap
        let param = h.param(0).expect("No parameter given").value();
        println!("param is: {:?}", param);
        let date: chrono::naive::date::NaiveDate = 
            serde_json::value::from_value(param.clone())
            .expect("Cannot create value from param");
        let rendered = format!("{}", date.format("%B %d, %Y"));
        try!(rc.writer.write(rendered.into_bytes().as_ref()));
        Ok(())
    }

    let mut hb = Handlebars::new();
    hb.register_helper("date", Box::new(date_helper));
    let mut hbse = HandlebarsEngine::from(hb);

    // CR mrussell: configurable tempate dir
    hbse.add(Box::new(DirectorySource::new("/Users/mrussell/code/rust/tv-trackr/templates/", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

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

    let shows: Vec<Show> = show_files.iter().map(|basename| {
        let file = format!("/Users/mrussell/code/tv-trackr/show-episodes/{}", basename);
        use scraped_show;
        let scraped_show: scraped_show::S = scraped_show::load(&file)
            .map_err(|e| format!("Could not load scraped show {}, err: {}", file, e))
            .unwrap();
        use std::convert::TryFrom;
        Show::try_from(scraped_show).unwrap()
    }).collect();

    let mut chain = Chain::new(handle_shows);
    chain.link_before(Read::<Shows>::one(shows));
    chain.link_after(hbse);
    chain
}


