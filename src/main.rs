#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use env_logger;
use hyper::server::Http;
use log::info;

use microservice::Microservice;

mod database;
mod functions;
mod html;
mod microservice;
mod models;
mod schema;

fn main() {
    env_logger::init();

    let address = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new().bind(&address, || Ok(Microservice {})).unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
