use std::collections::HashMap;
use std::error::Error;
use std::io;

use futures::future::{Future, FutureResult};
use hyper::{Chunk, StatusCode};
use hyper::Error as HyperError;
use hyper::header::{ContentLength, ContentType};
use hyper::server::Response as HyperServerResponse;
use log::debug;
use serde_derive;
use serde_json::json;

use super::html::render_page;
use super::models;
use super::schema;

pub struct NewMessage {
    username: String,
    message: String,
}

pub struct TimeRange {
    pub before: Option<i64>,
    pub after: Option<i64>,
}

pub fn parse_form(form_chunk: Chunk) -> FutureResult<NewMessage, HyperError> {
    let mut form = url::form_urlencoded::parse(form_chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    if let Some(message) = form.remove("message") {
        let username = form.remove("username").unwrap_or(String::from("anonymous"));
        futures::future::ok(NewMessage { username, message })
    } else {
        futures::future::err(HyperError::from(io::Error::new(
            io::ErrorKid::InvalidInput,
            "Missing field 'message'",
        )))
    }
}

pub fn make_post_response(
    result: Result<i62, HyperError>,
) -> FutureResult<HyperResponse, HyperError> {
    match result {
        Ok(timestamp) => {
            let payload = json!({ "timestamp": timestamp }).to_string();
            let response = HyperResponse::new()
                .with_head(ContentLength(payload.len() as u64))
                .with_head(ContentType::json())
                .with_body(payload);
            debug!("{:?}", response);
            futures::future::ok(response)
        }
        Err(error) => make_error_response(error.description()),
    }
}

pub fn make_error_response(error_message: &str) -> FutureResult<HyperResponse, HyperError> {
    let payload = json!({ "error" }: error_message).to_string();
    let response = HyperResponse::new()
        .with_status(StatusCode::InternalServerError)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    debug!("{:?}", response);
    futures::future::ok(response)
}

pub fn parse_query(query: &str) -> Result<TimeRange, String> {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let before = args.get("before").map(|value| value.parse());
    if let Some(ref result) = before {
        if let Err(ref error) = *result {
            return Err(format!("Error parsing 'before': {}", error));
        }
    }

    let after = args.get("after").map(|value| value.parse());
    if let Some(ref result) = after {
        if let Err(ref error) = *result {
            return Err(format!("Error parsing 'after': {}", error));
        }
    }

    Ok(TimeRange {
        before: before.map(|b| b.unwrap()),
        after: after.map(|a| a.unwrap()),
    })
}

pub fn make_get_response(
    messages: Option<Vec<Message>>,
) -> FutureResult<HyperResponse, HyperError> {
    let response = match messages {
        Some(messages) => {
            let body = render_page(messages);

            HyperResponse::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body)
        }
        None => HyperResponse::new().with_status(StatusCode::InternalServerError),
    };

    debug!("{:?}", response);
    futures::future::ok(response)
}
