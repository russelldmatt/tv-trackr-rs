use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};

use serde_json;

fn handle1(_: &mut Request) -> IronResult<Response> {
    println!("template");
    let mut response = Response::new();

    let data = ();
    response.set_mut(Template::new("test", data)).set_mut(status::Ok);
    println!("{}", response);
    Ok(response)
}

fn handle2(_: &mut Request) -> IronResult<Response> {
    println!("template2");

    #[derive(Serialize, Deserialize, Debug)]
    struct Data {
        text: String
    }

    use rustc_serialize::json::{Json, ToJson};
    impl ToJson for Data {
        fn to_json(&self) -> Json { 
            Json::from_str(serde_json::to_string(&self).unwrap().as_ref()).unwrap()
        }
    }

    let mut response = Response::new();

    let data = Data { text: "text of struct".to_string() };
    response.set_mut(Template::new("test2", data)).set_mut(status::Ok);
    println!("{}", response);
    Ok(response)
}

fn handle_shows(_: &mut Request) -> IronResult<Response> {
    // use chrono::date::Date;
    // use chrono::offset::local::Local;
    // use chrono::TimeZone;

    println!("shows");

    #[derive(Serialize, Deserialize, Debug)]
    struct Show {
        name: String,
        episodes: Vec<Episode>,
    }
    
    // CR mrussell: fix
    #[derive(Serialize, Deserialize, Debug)]
    struct Date {
        year: i32,
        month: i32,
        day: i32,
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Episode {
        name: String,
        season: i32,
        episode: i32,
        aire_date: Date,
        seen_class: String,
    }

    let mut response = Response::new();

    let ballers = 
        Show {
            name: "Ballers".to_string(),
            episodes: vec![
                Episode { 
                    name:"Game Day".to_string(),
                    season: 2,
                    episode: 10,
                    aire_date: Date { year: 2016, month: 9, day:25 },
                    seen_class: "seen".to_string()
                }
            ]
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
    let mut hbse = HandlebarsEngine::new();
    // CR mrussell: configurable tempate dir
    hbse.add(Box::new(DirectorySource::new("/Users/mrussell/code/rust/tv-trackr/templates/", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }
    // let mut chain = Chain::new(handle);
    let _ = handle1;
    let _ = handle2;
    let mut chain = Chain::new(handle_shows);
    chain.link_after(hbse);
    chain
}


