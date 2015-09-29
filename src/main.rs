#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate rustful;
extern crate serde;
extern crate serde_json;

use rustful::Method::{Get, Post};
use std::io::{BufReader, Read};
use std::error::Error;
use rustful::{Server, Context, Response, Router, TreeRouter};

#[derive(Serialize, Deserialize, Debug)]
struct Password {
    id: i32,
    encrypted: String,
}


fn get_passwords(context: Context, response: Response) {
    let password = Password { id: 1, encrypted: "erwerzxc".to_string() };
    let payload = serde_json::to_string(&password).unwrap();

    response.send(payload);
}

fn create_password(context: Context, response: Response) {
    let mut buffer = String::new();
    BufReader::new(context.body).read_to_string(&mut buffer).unwrap();
    let password: Password = serde_json::from_str(&buffer).unwrap();
    let payload = serde_json::to_string(&password).unwrap();

    response.send(payload);
}

fn main() {
    let mut router = TreeRouter::new();
    // needed to avoid
    // expected `fn(rustful::context::Context<'_, '_, '_>, rustful::response::Response<'_, '_>) {get_passwords}`,
    // found `fn(rustful::context::Context<'_, '_, '_>, rustful::response::Response<'_, '_>) {create_password}
    found `fn(rustful::context::Context<'_, '_, '_>, rustful::response::Response<'_, '_>) {create_password}`
    router.insert(Get, &"passwords", get_passwords as fn(Context, Response));
    router.insert(Post, &"passwords", create_password as fn(Context, Response));

    let server_result = Server {
        host: 8080.into(),
        handlers: router,
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}
