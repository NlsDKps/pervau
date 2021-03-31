use chrono::{NaiveDate, NaiveTime};
use log::error;
use std::io::stdin;

/// Read string from console
pub fn read_string() -> Option<String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            let retval = input[..(input.len()-1)].to_string(); // Drop newline
            Some(retval)
        },
        Err(e) => {
            error!("Could not read line: {}", e);
            None
        }
    }
}

/// Read date from console
pub fn read_date() -> Option<NaiveDate> {
    let s_date = match read_string() {
        Some(s_date) => s_date,
        None => return None
    };
    match NaiveDate::parse_from_str(&s_date, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(e) => {
            error!("Could not parse string to date: {}", e);
            None
        }
    }
}

/// Read time from console
pub fn read_time() -> Option<NaiveTime> {
    let s_time = match read_string() {
        Some(s_time) => s_time,
        None => return None
    };
    match NaiveTime::parse_from_str(&s_time, "%H:%M") {
        Ok(time) => Some(time),
        Err(e) => {
            error!("Could not parse string to time: {}", e);
            None
        }
    }
}

/// Read i32 from console
pub fn read_i32() -> Option<i32> {
    let s_number = match read_string() {
        Some(s_number) => s_number,
        None => return None
    };
    match s_number.parse::<i32>() {
        Ok(number) => Some(number),
        Err(e) => {
            error!("Could not parse string to i32: {}", e);
            None
        }
    }
}

/// Read f64 from console
pub fn read_f64() -> Option<f64> {
    let s_number = match read_string() {
        Some(s_number) => s_number,
        None => return None
    };
    match s_number.parse::<f64>() {
        Ok(number) => Some(number),
        Err(e) => {
            error!("Could not parse string to f64: {}", e);
            None
        }
    }
}

