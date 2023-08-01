

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;


pub fn get_date_time()->String{
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    return format!("{}", datetime.format("%d/%m/%Y %T"));
}
