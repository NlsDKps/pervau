use log::error;
use pervau::{
    controller::{
        database::local_conn_string,
        management::bloodpressure as mgmt_bp,
    },
};
use std::env::args;

fn delete_bp_by_id<'a>(db_url: &'a str, id: &'a i32) -> () {
    match mgmt_bp::delete(db_url, id) {
        true => println!("Success"),
        false => println!("Failure")
    }
}

fn parse_id<'a>() -> Option<i32> {
    match args().nth(1) {
        Some(id) => {
            match id.parse::<i32>() {
                Ok(id) => Some(id),
                Err(e) => {
                    error!("Could not parse id: {}", e);
                    None
                }
            }
        },
        None => None
    }
}

fn main() -> () {
    env_logger::init();
    let db_url = match local_conn_string() {
        Some(db_url) => db_url,
        None => return
    };
    let id = match parse_id() {
        Some(id) => id,
        None => return
    };
    delete_bp_by_id(&db_url, &id)
}
