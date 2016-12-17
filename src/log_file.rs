use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use iron::typemap::Key;

use persistent::{Read};

pub fn handler() -> impl Handler {
    #[derive(Copy, Clone)]
    pub struct Log;
    impl Key for Log { type Value = String; }

    fn serve_hits(req: &mut Request) -> IronResult<Response> {
        println!("serve hits");
        let arc = req.get::<Read<Log>>().unwrap();
        let log_path = arc.as_ref();
        Ok(Response::with((status::Ok, format!("Hits: {:?}", log_path))))
    }

    // This can be passed from command line arguments for example.
    let log_path = String::from("/path/to/a/log/file.log");
    let mut chain = Chain::new(serve_hits);
    
    // chain.link(Read::<Log>::both(log_path));
    chain.link_before(Read::<Log>::one(log_path));
    chain
}

