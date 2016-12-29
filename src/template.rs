use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};
use serde_json;

use iron::typemap::Key;
use persistent::{Read};

use show::*;

#[derive(Copy, Clone)]
pub struct Shows;
impl Key for Shows { type Value = Vec<Show>; }


fn handle_shows(req: &mut Request) -> IronResult<Response> {
    println!("shows");

    let mut response = Response::new();
    let shows = req.get::<Read<Shows>>().unwrap();
    let shows = shows.as_ref();
    
    #[derive(Serialize, Debug)]
    struct Data<'a> { 
        shows: &'a Vec<Show>,
    }

    let data = Data { shows };
    response.set_mut(Template::new("episodes", data)).set_mut(status::Ok);
    println!("{}", response);
    Ok(response)
}

pub fn handler() -> impl Handler {
    // helpers
    // https://github.com/sunng87/handlebars-rust#extensible-helper-system
    use handlebars::{Helper, Handlebars, Context, RenderContext, RenderError};
    use chrono;
    fn date_helper (_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext)
                    -> Result<(), RenderError> {
        // just for example, add error check for unwrap
        let param = h.param(0).unwrap().value();
        let date: chrono::naive::date::NaiveDate = 
            serde_json::value::from_value(param.clone()).unwrap();
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


