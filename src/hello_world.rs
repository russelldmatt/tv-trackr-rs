use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

pub fn handler() -> impl Handler {
    #[derive(Serialize, Deserialize)]
    struct Greeting {
        msg: String
    }

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        use serde_json;
        println!("hello world in hello_world.rs");
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let json_greeting = serde_json::value::to_value(greeting);
        let json_greeting_string: String = format!("{}", json_greeting);
        // let payload = json::encode(&json_greeting_string).unwrap();
        Ok(Response::with((status::Ok, json_greeting_string)))
    }

    Chain::new(hello_world)
}
