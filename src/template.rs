use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};
use serde_json;

fn handle_shows(_: &mut Request) -> IronResult<Response> {
    println!("shows");
    use show::*;
    use chrono;
    
    fn date_of_string(date_str: &str) -> chrono::naive::date::NaiveDate {
        use chrono::format;
        let mut parsed = format::parsed::Parsed::new();
        let format_string = format::strftime::StrftimeItems::new("%B %d, %Y");
        format::parse(&mut parsed, date_str, format_string).unwrap();
        parsed.to_naive_date().unwrap()
    }

    let mut response = Response::new();
    let ballers = 
        Show {
            name: "Ballers".to_string(),
            episodes: {
                vec![
                    Episode { 
                        id: 1,
                        name:"Game Day".to_string(),
                        season: 2,
                        episode: 10,
                        aire_date: date_of_string("September 25, 2016"),
                        seen_class: "seen".to_string()
                    },
                    Episode { 
                        id: 2,
                        name:"Million Bucks in a Bag".to_string(),
                        season: 2,
                        episode: 9,
                        aire_date: date_of_string("September 25, 2016"),
                        seen_class: "seen".to_string()
                    },
                ]
            }
        };
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Data { 
        shows: Vec<Show>,
    }

    let data = Data { shows: vec![ballers] };
    response.set_mut(Template::new("episodes", data)).set_mut(status::Ok);
    println!("{}", response);
    Ok(response)
}

pub fn handler() -> impl Handler {
    // helpers
    // https://github.com/sunng87/handlebars-rust#extensible-helper-system
    use handlebars::{Helper, Handlebars, Context, RenderContext, RenderError};
    use chrono;
    fn date_helper (_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
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
    let mut chain = Chain::new(handle_shows);
    chain.link_after(hbse);
    chain
}


