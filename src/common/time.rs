use chrono::{NaiveDateTime, Local};

pub fn format_time(time: NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn parse_time_str(time_str: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(time_str.as_str(), "%Y-%m-%d %H:%M:%S").unwrap()
}

pub fn now_time_str() -> String {
    format_time(Local::now().naive_local())
}
