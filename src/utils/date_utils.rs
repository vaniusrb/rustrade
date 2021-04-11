use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

/// Convert numeric date to String iso formatted
pub fn timestamp_to_datetime(timestamp: &u64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp((timestamp / 1000) as i64, 0);
    DateTime::from_utc(naive, Utc)
}

/// Convert numeric date to String iso formatted
pub fn _timestamp_to_str(timestamp: &u64) -> String {
    let date_time: DateTime<Utc> = timestamp_to_datetime(timestamp);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_timestamp(date_time: &DateTime<Utc>) -> u64 {
    date_time.timestamp_millis() as u64
}

pub fn _datetime_to_str(date_time: &DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_filename(date_time: &DateTime<Utc>) -> String {
    date_time.format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn time_to_str(date_time: &DateTime<Utc>) -> String {
    date_time.format("%H:%M:%S").to_string()
}

/// Convert string in format YYYY-MM-DD HH:MM:SS to DateTime<Utc>
pub fn str_to_datetime(string: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(string, "%Y-%m-%d %H:%M:%S").unwrap()
}

pub fn str_d(string: &str) -> DateTime<Utc> {
    str_to_datetime(string)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn timestamp_to_str_test() {
        let dtu = Utc.ymd(1979, 1, 13).and_hms(11, 30, 0);
        let dts = "1979-01-13 11:30:00";
        assert_eq!(_datetime_to_str(&dtu), dts);
        assert_eq!(str_to_datetime(&dts), dtu);
        let dtm = datetime_to_timestamp(&dtu);
        assert_eq!(_timestamp_to_str(&dtm), dts);
    }
}
