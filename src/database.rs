use std::env;
use std::io;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::future::FutureResult;
use hyper::Error as HyperError;
use log::error;

use super::models::{Message, NewMessage, TimeRange};

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

pub fn write_to_db(
    new_message: NewMessage,
    db_connection: &PgConnection,
) -> FutureResult<i64, HyperError> {
    use crate::schema::messages;

    let timestamp = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(messages::timestamp)
        .get_result(db_connection);

    match timestamp {
        Ok(timestamp) => futures::future::ok(timestamp),
        Err(error) => {
            error!("Error writing to the database: {}", error);
            futures::future::err(HyperError::from(io::Error::new(
                io::ErrorKind::Other,
                "service error",
            )))
        }
    }
}

pub fn connect_to_db() -> Option<PgConnection> {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL));

    match PgConnection::establish(&database_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            error!("Error connecting to the database: {}", error);
            None
        }
    }
}

pub fn query_db(time_range: TimeRange, db_connection: &PgConnection) -> Option<Vec<Message>> {
    use crate::schema::messages;

    let TimeRange { before, after } = time_range;

    let mut query = messages::table.into_boxed();

    if let Some(before) = before {
        query = query.filter(messages::timestamp.lt(before as i64))
    }

    if let Some(after) = after {
        query = query.filter(messages::timestamp.gt(after as i64))
    }

    let query_result = query.load::<Message>(db_connection);

    match query_result {
        Ok(result) => Some(result),
        Err(error) => {
            error!("Error querying DB: {}", error);
            None
        }
    }
}
