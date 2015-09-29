#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate serde;
extern crate serde_json;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use persistent::Read as PRead; // needed to disambiguate with std::io::Read
use router::Router;

mod db;
mod dal;


fn get_passwords(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PRead<db::PostgresDB>>().unwrap();
    let conn = pool.get().unwrap();
    match dal::list_passwords(conn) {
        Ok(passwords) => {
            let response_payload = serde_json::to_string(&passwords).unwrap();
            Ok(Response::with((status::Ok, response_payload)))
        },
        Err(err) => {
            println!("{:?}", err);
            Ok(Response::with((status::InternalServerError)))
        }
    }
}

fn create_password(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let password: dal::Password = serde_json::from_str(&payload).unwrap();

    let response_payload = serde_json::to_string(&password).unwrap();

    Ok(Response::with((status::Ok, response_payload)))
}


fn main() {
    let mut router = Router::new();
    router.get("/passwords", get_passwords);
    router.post("/passwords", create_password);

    let pool = db::get_pool("postgres://pg:pg@localhost:5432/safe");

    // Cleanup and set some seed data
    db::setup_database(pool.get().unwrap());

    let mut chain = Chain::new(router);
    chain.link(PRead::<db::PostgresDB>::both(pool));

    Iron::new(chain).http("localhost:5000").unwrap();
    println!("Listening on 5000");
}
