use crate::services::provider::candles_range::minutes_open_trunc;
use crate::str_to_datetime;
use crate::utils::date_utils::str_d;
use chrono::{DateTime, Duration, Timelike, Utc};
use eyre::bail;
use std::{cmp::Ordering, convert::TryFrom, fmt, ops::Add, ops::Sub};

#[derive(Debug, Eq, Copy, Clone)]
pub enum OpenCloseTime {
    Open(DateTime<Utc>),
    Close(DateTime<Utc>),
    OpenClose(DateTime<Utc>, DateTime<Utc>),
}

impl PartialEq for OpenCloseTime {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OpenCloseTime::Open(self_open), OpenCloseTime::Open(other_open)) => {
                self_open == other_open
            }
            (OpenCloseTime::Open(_self_open), OpenCloseTime::Close(_other_close)) => false,
            (
                OpenCloseTime::Open(self_open),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => self_open == other_open,
            (OpenCloseTime::Close(_self_close), OpenCloseTime::Open(_other_open)) => false,
            (OpenCloseTime::Close(self_close), OpenCloseTime::Close(other_close)) => {
                self_close == other_close
            }
            (
                OpenCloseTime::Close(self_close),
                OpenCloseTime::OpenClose(_other_open, other_close),
            ) => self_close == other_close,
            (OpenCloseTime::OpenClose(self_open, _self_close), OpenCloseTime::Open(other_open)) => {
                self_open == other_open
            }
            (
                OpenCloseTime::OpenClose(_self_open, self_close),
                OpenCloseTime::Close(other_close),
            ) => self_close == other_close,
            (
                OpenCloseTime::OpenClose(self_open, _sc),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => self_open == other_open,
        }
    }
}
impl Ord for OpenCloseTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (OpenCloseTime::Open(self_open), OpenCloseTime::Open(other_open)) => {
                self_open.cmp(other_open)
            }
            (OpenCloseTime::Open(_self_open), OpenCloseTime::Close(_other_close)) => {
                Ordering::Equal
            }
            (
                OpenCloseTime::Open(self_open),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => self_open.cmp(other_open),
            (OpenCloseTime::Close(_self_close), OpenCloseTime::Open(_other_open)) => {
                Ordering::Equal
            }
            (OpenCloseTime::Close(self_close), OpenCloseTime::Close(other_close)) => {
                self_close.cmp(other_close)
            }
            (
                OpenCloseTime::Close(self_close),
                OpenCloseTime::OpenClose(_other_open, other_close),
            ) => self_close.cmp(other_close),
            (OpenCloseTime::OpenClose(self_open, _self_close), OpenCloseTime::Open(other_open)) => {
                self_open.cmp(other_open)
            }
            (
                OpenCloseTime::OpenClose(_self_open, self_close),
                OpenCloseTime::Close(other_close),
            ) => self_close.cmp(other_close),
            (
                OpenCloseTime::OpenClose(self_open, _sc),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => self_open.cmp(other_open),
        }
    }
}

impl PartialOrd for OpenCloseTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (OpenCloseTime::Open(self_open), OpenCloseTime::Open(other_open)) => {
                Some(self_open.cmp(other_open))
            }
            (OpenCloseTime::Open(_self_open), OpenCloseTime::Close(_other_close)) => None,
            (
                OpenCloseTime::Open(self_open),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => Some(self_open.cmp(other_open)),
            (OpenCloseTime::Close(_self_close), OpenCloseTime::Open(_other_open)) => None,
            (OpenCloseTime::Close(self_close), OpenCloseTime::Close(other_close)) => {
                Some(self_close.cmp(other_close))
            }
            (
                OpenCloseTime::Close(self_close),
                OpenCloseTime::OpenClose(_other_open, other_close),
            ) => Some(self_close.cmp(other_close)),
            (OpenCloseTime::OpenClose(self_open, _self_close), OpenCloseTime::Open(other_open)) => {
                Some(self_open.cmp(other_open))
            }
            (
                OpenCloseTime::OpenClose(_self_open, self_close),
                OpenCloseTime::Close(other_close),
            ) => Some(self_close.cmp(other_close)),
            (
                OpenCloseTime::OpenClose(self_open, _sc),
                OpenCloseTime::OpenClose(other_open, _other_close),
            ) => Some(self_open.cmp(other_open)),
        }
    }
}

impl fmt::Display for OpenCloseTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenCloseTime::Open(open) => write!(f, "{}", open),
            OpenCloseTime::Close(close) => write!(f, "{}", close),
            OpenCloseTime::OpenClose(open, _close) => write!(f, "{}", open),
        }
    }
}

impl TryFrom<&str> for OpenCloseTime {
    type Error = eyre::Error;

    fn try_from(value: &str) -> eyre::Result<Self> {
        if value.is_empty() {
            bail!("GreaterThanZero only accepts value superior than zero!")
        } else {
            let date = str_d(value);
            Ok(if date.second() == 59 {
                OpenCloseTime::Close(date)
            } else {
                OpenCloseTime::Open(date)
            })
        }
    }
}

impl OpenCloseTime {
    pub fn to_dates(&self, minutes: i32) -> (DateTime<Utc>, DateTime<Utc>) {
        match self {
            OpenCloseTime::OpenClose(o, c) => (*o, *c),
            OpenCloseTime::Open(o) => (
                *o,
                *o + Duration::minutes(minutes as i64) - Duration::seconds(1),
            ),
            OpenCloseTime::Close(c) => (
                *c - Duration::minutes(minutes as i64) + Duration::seconds(1),
                *c,
            ),
        }
    }

    pub fn open(&self, minutes: i32) -> DateTime<Utc> {
        self.to_dates(minutes).0
    }

    pub fn close(&self, minutes: i32) -> DateTime<Utc> {
        self.to_dates(minutes).1
    }

    pub fn from_date(date_time: &DateTime<Utc>, minutes: i32) -> OpenCloseTime {
        let open = minutes_open_trunc(date_time, minutes);
        let close = open + Duration::minutes(minutes as i64) - Duration::seconds(1);
        OpenCloseTime::OpenClose(open, close)
    }

    pub fn from_date_close(close: &DateTime<Utc>, minutes: i32) -> OpenCloseTime {
        let close = *close;
        let open = close + Duration::seconds(1) - Duration::minutes(minutes as i64);
        OpenCloseTime::OpenClose(open, close)
    }

    pub fn from_str(date_time: &str, minutes: i32) -> OpenCloseTime {
        let open = minutes_open_trunc(&str_d(date_time), minutes);
        let close = open + Duration::minutes(minutes as i64) - Duration::seconds(1);
        OpenCloseTime::OpenClose(open, close)
    }
}

impl Add<Duration> for OpenCloseTime {
    type Output = OpenCloseTime;

    fn add(self, other: Duration) -> OpenCloseTime {
        match self {
            OpenCloseTime::Open(open) => OpenCloseTime::Open(open + other),
            OpenCloseTime::Close(close) => OpenCloseTime::Close(close + other),
            OpenCloseTime::OpenClose(open, close) => {
                OpenCloseTime::OpenClose(open + other, close + other)
            }
        }
    }
}

impl Sub<Duration> for OpenCloseTime {
    type Output = OpenCloseTime;

    fn sub(self, other: Duration) -> OpenCloseTime {
        match self {
            OpenCloseTime::Open(open) => OpenCloseTime::Open(open - other),
            OpenCloseTime::Close(close) => OpenCloseTime::Close(close - other),
            OpenCloseTime::OpenClose(open, close) => {
                OpenCloseTime::OpenClose(open - other, close - other)
            }
        }
    }
}

pub fn _str_close(date_time: &str) -> OpenCloseTime {
    OpenCloseTime::Close(str_to_datetime(date_time))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn open_close_test() {
        let open_close = OpenCloseTime::try_from("2020-01-20 00:00:00").unwrap();
        let date = str_d("2020-01-20 00:00:00");
        assert_eq!(open_close, OpenCloseTime::Open(date));

        let open_close = OpenCloseTime::try_from("2020-01-20 00:00:00").unwrap();
        let date = str_d("2020-01-20 00:00:00");
        assert_eq!(open_close, OpenCloseTime::Open(date));
    }

    #[test]
    fn compare_open_close_test() {
        let open_close_1 = OpenCloseTime::try_from("2020-01-20 00:00:00").unwrap();
        let open_close_2 = OpenCloseTime::try_from("2020-01-20 10:00:00").unwrap();

        assert!(open_close_2 > open_close_1);
        assert!(open_close_1 < open_close_2);
        assert!(open_close_1 != open_close_2);
    }
}
