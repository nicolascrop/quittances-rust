extern crate chrono;
use chrono::{Local, Datelike};
use self::chrono::TimeZone;
use std::io;

pub fn get_default_date() -> String {
    let local_time = Local::now();
    let mut year = local_time.year();
    let mut month = local_time.month();
    if month == 1 {
        year = year - 1;
        month = 12;
    }
    let default_date = Local.ymd(year, month, 1).format("%Y%m").to_string();

    println!("No receipt number received, rrm will use {} as value", default_date);
    println!("write y to continue");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if input.as_str() != "y" {
               std::process::exit(0);
            }
        }
        Err(error) => println!("error: {}", error)
    }

    return default_date;
}
