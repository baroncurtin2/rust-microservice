use futures;
use futures::future::Future;
use futures::Stream;
use hyper::server::{Request as HyperRequest, Response as HyperResponse, Service as HyperService};
use hyper::Error as HyperError;
use hyper::Method::{Get, Post};
use hyper::StatusCode;

use super::database::{connect_to_db, query_db, write_to_db};
use super::functions::{
    make_error_response, make_get_response, make_post_response, parse_form, parse_query,
};
use super::models::TimeRange;

pub struct Microservice;

impl HyperService for Microservice {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = HyperError;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: HyperRequest) -> Self::Future {
        let db_connection = match connect_to_db() {
            Some(connection) => connection,
            None => {
                return Box::new(futures::future::ok(
                    HyperResponse::new().with_status(StatusCode::InternalServerError),
                ))
            }
        };

        match (request.method(), request.path()) {
            (&Post, "/") => {
                let future = request
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(move |new_message| write_to_db(new_message, &db_connection))
                    .then(make_post_response);
                Box::new(future)
            }
            (&Get, "/") => {
                let time_range = match request.query() {
                    Some(query) => parse_query(query),
                    None => Ok(TimeRange {
                        before: None,
                        after: None,
                    }),
                };
                let response = match time_range {
                    Ok(time_range) => make_get_response(query_db(time_range, &db_connection)),
                    Err(error) => make_error_response(&error),
                };
                Box::new(response)
            }
            _ => Box::new(futures::future::ok(
                HyperResponse::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}
