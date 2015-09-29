#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate rustful;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use rustful::{Server, Context, Response, TreeRouter};

#[derive(Serialize, Deserialize)]
struct Password {
    id: i32,
    encrypted: String,
}


fn get_passwords(context: Context, response: Response) {
    let password = Password { id: 1, encrypted: "erwerzxc".to_string() };
    let payload = serde_json::to_string(&password).unwrap();

    response.send(payload);
}

fn main() {
    let routes = insert_routes!{
        TreeRouter::new() => {
            "passwords" => {
                Get: get_passwords,
            }
        }
    };

    let server_result = Server {
        host: 8080.into(),
        handlers: routes,
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}
