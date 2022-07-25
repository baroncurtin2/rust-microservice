use hyper::server::Http;
use log::info;

use microservice::Microservice;

mod database;
mod html;
mod message;
mod microservice;
mod models;
mod schema;

fn main() {
    env_logger::init();

    let address = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
