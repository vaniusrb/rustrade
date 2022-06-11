use super::open_close_time::OpenCloseTime;
use chrono::DateTime;
use chrono::Utc;
use eyre::bail;
use eyre::Result;

#[derive(Debug, Clone, Copy)]
pub struct OpenCloseRange(pub OpenCloseTime, pub OpenCloseTime);

impl OpenCloseRange {
    pub fn from_open_close(start: OpenCloseTime, end: OpenCloseTime) -> Result<Self> {
        if start > end {
            bail!("Start date time {:?} is greater than end {:?}!", start, end);
        }
        Ok(Self(start, end))
    }

    pub fn from_dates(start: DateTime<Utc>, end: DateTime<Utc>, minutes: i32) -> Result<Self> {
        Self::from_open_close(
            OpenCloseTime::from_date(&start, minutes),
            OpenCloseTime::from_date(&end, minutes),
        )
    }

    pub fn open_close(&self) -> (OpenCloseTime, OpenCloseTime) {
        (self.0, self.1)
    }
}
