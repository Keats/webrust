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

#[macro_use]
mod db;
mod dal;


macro_rules! try_or_500 {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(e) => {
            println!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)))
        }
    })
}

fn get_passwords(req: &mut Request) -> IronResult<Response> {
    let conn = get_pg_connection!(req);
    match dal::list_passwords(conn) {
        Ok(passwords) => {
            let response_payload = serde_json::to_string(&passwords).unwrap();
            Ok(Response::with((status::Ok, response_payload)))
        },
        Err(e) => {
            println!("Errored: {:?}", e);
            Ok(Response::with((status::InternalServerError)))
        }
    }
}

fn create_password(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    try_or_500!(req.body.read_to_string(&mut payload));
    let password: dal::Password  = try_or_500!(serde_json::from_str(&payload));

    let conn = get_pg_connection!(req);

    match dal::create_password(conn, password) {
        Ok(_) => Ok(Response::with((status::Created))),
        Err(e) => {
            println!("Errored: {:?}", e);
            Ok(Response::with((status::InternalServerError)))
        }
    }
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
