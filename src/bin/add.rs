use std::env::args;
use pervau::{
    controller::{
        database::local_conn_string,
        io::{
            read_date,
            read_i32,
            read_time,
        },
        management::bloodpressure as mgmt_bp,
    },
    model::serializable::bloodpressure::NewSerBloodPressure,
};

/// Prints a help text to console
fn print_help() -> () {
    let help_text = "\
Usage: add OPTION
Options are:
\t-b, --bloodpressure\tAdd bloodpressure measurement to database
    ";
    println!("{}", help_text)
}

/// Read a bloodpressure measurement from console and add to database
///
/// # Note
/// Since Sqlite does not support date and time, read_date and read_time are
/// just sanity-checks of the user entry.
fn add_bloodpressure<'a>(db_url: &'a str) -> () {
    println!("Supply date (YYYY-MM-DD):");
    let date = match read_date() {
        Some(date) => date.to_string(),
        None => return
    };
    println!("Supply time (HH:MM):");
    let time = match read_time() {
        Some(time) => time.to_string(),
        None => return
    };
    println!("Supply systole:");
    let systole = match read_i32() {
        Some(value) => value,
        None => return
    };
    println!("Supply diastole:");
    let diastole = match read_i32() {
        Some(value) => value,
        None => return
    };
    println!("Supply pulse:");
    let pulse = match read_i32() {
        Some(value) => value,
        None => return
    };
    let new_ser_bp = NewSerBloodPressure {
        date: date,
        time: time,
        systole: systole,
        diastole: diastole,
        pulse: pulse
    };
    match mgmt_bp::add(db_url, &new_ser_bp) {
        true => println!("Success"),
        false => println!("Failure")
    }
}

/// Parse arguments provided via shell
fn parse_arguments<'a>(db_url: &'a str) -> () {
    let argument = match args().nth(1) {
        Some(arg) => arg,
        None => String::from("")
    };
    match argument.as_str() {
        "-b" | "--bloodpressure" => add_bloodpressure(db_url),
        _ => print_help()
    }
}

/// Main function, called when starting the binary
fn main() -> () {
    env_logger::init();
    let db_url = match local_conn_string() {
        Some(db_url) => db_url,
        None => return
    };
    parse_arguments(&db_url);
}
