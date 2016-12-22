use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};
use serde_json;

fn handle_shows(_: &mut Request) -> IronResult<Response> {
    println!("shows");
    use show::*;
    let mut response = Response::new();
    let ballers = 
        Show {
            name: "Ballers".to_string(),
            episodes: {
                use time;
                vec![
                    {
                        let aire_date = Time(time::strptime("2016-09-25", "%Y-%m-%d").unwrap());
                        Episode { 
                            id: 1,
                            name:"Game Day".to_string(),
                            season: 2,
                            episode: 10,
                            aire_date: aire_date.clone(),
                            aire_date_string: aire_date.strftime("%B %d, %Y").unwrap().to_string(),
                            seen_class: "seen".to_string()
                        }
                    },
                    {
                        let aire_date = Time(time::strptime("2016-09-25", "%Y-%m-%d").unwrap());
                        Episode { 
                            id: 2,
                            name:"Million Bucks in a Bag".to_string(),
                            season: 2,
                            episode: 9,
                            aire_date: aire_date.clone(),
                            aire_date_string: aire_date.strftime("%B %d, %Y").unwrap().to_string(),
                            seen_class: "seen".to_string()
                        }
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
    let mut hbse = HandlebarsEngine::new();
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


