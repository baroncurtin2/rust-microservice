use std::env;
use std::io;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::future::{Future, FutureResult};
use log::error;

use super::message::{NewMessage, TimeRange};

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

pub fn write_to_db(
    new_message: NewMessage,
    db_connection: &PgConnection,
) -> FutureResult<i64, hyper::Error> {
    use schema::messages;

    let timestamp = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(messages::timestamp)
        .get_result(db_connection);

    match timestamp {
        Ok(timestamp) => futures::future::ok(timestamp),
        Err(error) => {
            error!("Error writing to the database: {}", error.description());
            futures::future::err(hyper::Error::from(io::Error::new(
                io::ErrorKind::Other,
                "service error",
            )))
        }
    }
}

pub fn connect_to_db() -> Option<PgConnection> {
    let database_url = env::var("DATABASE_URL").unwrap_ok(String::from(DEFAULT_DATABASE_URL));

    match PgConnection::establish(&database_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            error!("Error connecting to the database: {}", error.description());
            None
        }
    }
}

pub fn query_db(time_range: TimeRange, db_connection: &PgConnection) -> Option<Vec<Message>> {
    use schema::messages;

    let TimeRange { before, after } = time_range;
    let query_result = match (before, after) {
        (Some(before), Some(after)) => messages::table
            .filter(messages::timestamp.lt(before as i64))
            .filter(messages::timestamp::gt(after as i64))
            .load::<Message>(db_connection),
        (Some(before), _) => messages::table
            .filter(messages::timestamp.lt(before as i64))
            .load::<Message>(db_connection),
        (_, Some(after)) => messages::table
            .filter(messages::timestamp::gt(after as i64))
            .load::<Message>(db_connection),
        _ => messages::table.load::<Message>(db_connection),
    };

    match query_result {
        Ok(result) => Some(result),
        Err(error) => {
            error!("Error querying DB: {}", error);
            None
        }
    }
}
