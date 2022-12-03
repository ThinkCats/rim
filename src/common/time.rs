use chrono::NaiveDateTime;

pub fn format_time(time: NaiveDateTime) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}