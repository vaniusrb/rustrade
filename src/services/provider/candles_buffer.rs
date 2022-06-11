use crate::model::candle::Candle;
use crate::model::open_close_range::OpenCloseRange;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use core::ops::Bound::Included;
use eyre::eyre;
use std::collections::btree_map::Range;
use std::collections::BTreeMap;

pub struct CandlesBuffer {
    min_duration: Duration,
    minutes: i32,
    tree_map: BTreeMap<DateTime<Utc>, Candle>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

impl CandlesBuffer {
    pub fn new(minutes: i32) -> Self {
        Self {
            min_duration: Duration::minutes(minutes as i64),
            minutes,
            tree_map: BTreeMap::new(),
            start: None,
            end: None,
        }
    }

    pub fn push_candles(&mut self, candles: Vec<Candle>) -> eyre::Result<()> {
        let (start, end) =
            insert_into_buffer(candles, self.min_duration, self.end, &mut self.tree_map)?;
        self.start = Some(start);
        self.end = Some(end);
        Ok(())
    }

    pub fn start(&self) -> Option<DateTime<Utc>> {
        self.start
    }

    pub fn end(&self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn candles_from_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Vec<&Candle> {
        let r = self
            .tree_map
            .range((Included(start_time), Included(end_time)))
            .map(|(_, c)| c)
            .collect::<Vec<_>>();
        r
    }

    pub fn candles_range_iter(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Range<DateTime<Utc>, Candle> {
        self.tree_map
            .range((Included(start_time), Included(end_time)))
    }

    pub fn missing_ranges(
        &self,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> eyre::Result<Vec<OpenCloseRange>> {
        let mut result = Vec::new();
        if let Some(self_start) = self.start {
            let self_start_min_dur = self_start - self.min_duration;
            if start_time < &self_start_min_dur {
                result.push(OpenCloseRange::from_dates(
                    *start_time,
                    self_start_min_dur,
                    self.minutes,
                )?)
            }
        } else {
            result.push(OpenCloseRange::from_dates(*start_time, *end_time, self.minutes)?)
        }
        if let Some(self_end) = self.end {
            let self_end_min_dur = self_end + self.min_duration;
            if end_time > &self_end_min_dur {
                result.push(OpenCloseRange::from_dates(self_end_min_dur, *end_time, self.minutes)?)
            }
        }
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.tree_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn insert_into_buffer(
    candles: Vec<Candle>,
    min_duration: Duration,
    mut previous_open_opt: Option<DateTime<Utc>>,
    tree_map: &mut BTreeMap<DateTime<Utc>, Candle>,
) -> eyre::Result<(DateTime<Utc>, DateTime<Utc>)> {
    for c in candles {
        if let Some(previous_open) = previous_open_opt {
            if previous_open - c.open_time != min_duration {
                // TODO: review if it's still valid
                return Err(eyre!("Different minutes!"));
            }
        }
        previous_open_opt = Some(c.open_time);
        tree_map.insert(c.open_time, c);
    }

    let start = tree_map.first_key_value().map(|r| *r.0).unwrap();
    let end = tree_map.last_key_value().map(|r| *r.0).unwrap();
    Ok((start, end))
}
