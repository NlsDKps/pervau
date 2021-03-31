use log::error;
use pervau::{
    controller::{
        database::local_conn_string,
        management::bloodpressure as mgmt_bp,
    },
};
use std::env::{args, Args};

/// Prints a help text to console
fn print_help() -> () {
    let help_text = "\
Usage: export OPTION
Options are:
\t-i, --by-id ID\tAdd bloodpressure measurement to database
    ";
    println!("{}", help_text)
}


/// Returns a bloodpressure measure by it's id in json-format
fn read_bp_by_id<'a>(db_url: &'a str, pos: &'a usize) -> String {
    let id = match args().nth(*pos) {
        Some(id) => {
            match id.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    error!("Could not parse position: {}", e);
                    return String::from("")
                }
            }
        },
        None => return String::from("")
    };
    let bp = match mgmt_bp::read_by_id(db_url, &id) {
        Some(bp) => bp,
        None => return String::from("")
    };
    match serde_json::to_string(&bp) {
        Ok(json) => json,
        Err(e) => {
            error!("Could not parse bp: {}", e);
            String::from("")
        }
    }
}

/// Reads all bloodpressure measurements and returns them in JSON-format
fn read_all_bp<'a>(db_url: &'a str) -> String {
    let bp = mgmt_bp::read_all(db_url);
    match serde_json::to_string(&bp) {
        Ok(json) => json,
        Err(e) => {
            error!("Could not parse bp: {}", e);
            String::from("")
        }
    }
}

fn parse_arguments<'a>(db_url: &'a str, mut args: Args) -> () {
    let mut i = 1;
    let mut json = String::from("");
    while i < args.len() {
        match args.nth(i) {
            Some(arg) => {
                match arg.as_str() {
                    "-a" => json = read_all_bp(db_url),
                    "-i" | "--by-id" => {
                        i += 1;
                        json = read_bp_by_id(db_url, &i);
                    },
                    _ => print_help()
                }
            },
            None => ()
        };
        i+=1;
    }
    println!("{}", json);
}
fn main() -> () {
    env_logger::init();
    let db_url = match local_conn_string() {
        Some(db_url) => db_url,
        None => return
    };
    let args = args();
    parse_arguments(&db_url, args);
}
