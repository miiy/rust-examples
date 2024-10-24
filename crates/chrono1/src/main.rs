use chrono::prelude::*;

fn main() {
    datetime_example();
}

fn datetime_example() {
    let utc = Utc::now();
    println!("{utc:?}");

    let local = Local::now();
    println!("{local:?}");

    //
    let timestamp = Local::now().timestamp();
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
    println!("datetime: {:?}", datetime.to_rfc2822());
    println!("Year: {}", datetime.year());
    println!("Month: {}", datetime.month());
    println!("Day: {}", datetime.day());
    println!("Hour: {}", datetime.hour());
    println!("Minute: {}", datetime.minute());
    println!("Second: {}", datetime.second());
}
