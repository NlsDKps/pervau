pub mod bloodpressure;

use diesel::prelude::*;
use dotenv::dotenv;
use log::error;
use std::env;

/// Returns the database location based on the process's environment.
/// To statically provide the database location, create a file `.env`.
pub fn local_conn_string() -> Option<String> {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(db_str) => Some(db_str),
        Err(e) => {
            error!("Could not read database url: {}", e);
            None
        }
    }
}

/// Allows to connect to the database given by `database_url`.
pub fn connect_database(database_url: &str)
    -> Option<SqliteConnection>
{
    match SqliteConnection::establish(&database_url) {
        Ok(conn) => Some(conn),
        Err(e) => {
            error!("Could not connect to database: {}", e);
            None
        }
    }
}


