use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};
use serde_json;

fn handle_shows(_: &mut Request) -> IronResult<Response> {
    println!("shows");
    use show::*;
    use time_wrapper::Time as Time;
    let mut response = Response::new();
    let ballers = 
        Show {
            name: "Ballers".to_string(),
            episodes: {
                use time;
                vec![
                    Episode { 
                        id: 1,
                        name:"Game Day".to_string(),
                        season: 2,
                        episode: 10,
                        aire_date: Time(time::strptime("2016-09-25", "%Y-%m-%d").unwrap()),
                        seen_class: "seen".to_string()
                    },
                    Episode { 
                        id: 2,
                        name:"Million Bucks in a Bag".to_string(),
                        season: 2,
                        episode: 9,
                        aire_date: Time(time::strptime("2016-09-25", "%Y-%m-%d").unwrap()),
                        seen_class: "seen".to_string()
                    },
                ]
            }
        };
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Data { 
        shows: Vec<Show>,
    }

    use rustc_serialize::json::{Json, ToJson};
    impl ToJson for Data {
        fn to_json(&self) -> Json { 
            Json::from_str(serde_json::to_string(&self).unwrap().as_ref()).unwrap()
        }
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
    use time_wrapper::Time as Time;
    fn date_helper (_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        // just for example, add error check for unwrap
        let param = h.param(0).unwrap().value();
        let time: Time = serde_json::value::from_value(param.clone()).unwrap();
        let rendered = time.strftime("%B %d, %Y").unwrap().to_string();
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


