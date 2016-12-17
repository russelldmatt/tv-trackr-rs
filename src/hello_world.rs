use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;

pub fn handler() -> impl Handler {
    #[derive(RustcEncodable, RustcDecodable)]
    struct Greeting {
        msg: String
    }

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        println!("hello world in hello_world.rs");
        use rustc_serialize::json;
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Chain::new(hello_world)
}
