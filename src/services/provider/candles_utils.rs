use crate::model::{candle::Candle, open_close::OpenClose};
use crate::utils::date_utils::str_to_datetime;
use crate::utils::date_utils::timestamp_to_datetime;
use crate::utils::dec_utils::fdec;
use binance::model::KlineSummary;
use chrono::Duration;
use ta::DataItem;

/// Convert binance Kline to TA DataItem
pub fn _kline_to_data_item(summary: &KlineSummary) -> DataItem {
    DataItem::builder()
        .open(summary.open)
        .high(summary.high)
        .low(summary.low)
        .close(summary.close)
        .volume(summary.volume)
        .build()
        .unwrap()
}

/// Convert binance Kline to app Candle
pub fn kline_to_candle(summary: &KlineSummary, symbol: i32, minutes: i32, id: i32) -> Candle {
    let open_time = timestamp_to_datetime(&(summary.open_time as u64));
    let close_time = timestamp_to_datetime(&(summary.close_time as u64));
    Candle {
        id,
        symbol,
        minutes,
        open: fdec(summary.open),
        open_time,
        high: fdec(summary.high),
        low: fdec(summary.low),
        close: fdec(summary.close),
        volume: fdec(summary.volume),
        close_time,
    }
}

/// If candles are sorted ok
pub fn _candles_sorted_ok(candles: &[&Candle]) -> bool {
    let sort_ok = candles
        .iter()
        .map(Some)
        .fold((true, None::<&&Candle>), |previous, current| {
            let result = if let Some(previous_c) = previous.1 {
                if let Some(current_c) = current {
                    previous.0 && (current_c.open_time > previous_c.open_time)
                } else {
                    previous.0
                }
            } else {
                previous.0
            };
            (result, current)
        });
    sort_ok.0
}

/// Returns inconsistent candles
pub fn inconsistent_candles(candles: &[&Candle], duration: &Duration) -> Vec<Candle> {
    candles
        .iter()
        .map(Some)
        .fold((Vec::new(), None::<&&Candle>), |mut previous, current| {
            if let Some(previous_c) = previous.1 {
                if let Some(current_c) = current {
                    let previous_d = previous_c.open_time;
                    let current_d = current_c.open_time;
                    if current_d - previous_d != *duration {
                        previous.0.push(*(*current_c));
                    }
                }
            };
            (previous.0, current)
        })
        .0
}

/// Returns min/max close time from candles list
pub fn min_max_close_time_from_candles(candles: &[&Candle]) -> Option<(OpenClose, OpenClose)> {
    if candles.is_empty() {
        return None;
    }
    let mut min_date = OpenClose::Open(str_to_datetime("2000-01-01 00:00:00"));
    let max_date = candles
        .iter()
        .map(|c| c.open_close())
        .fold(min_date, |acc, x| acc.max(x));
    min_date = candles
        .iter()
        .map(|c| c.open_close())
        .fold(max_date, |acc, x| acc.min(x));
    Some((min_date, max_date))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils::dec_utils::fdec;
    use chrono::Duration;

    #[test]
    fn candles_sorted_ok_test() {
        let c1 = Candle::new(
            0,
            "2020-01-12 12:00:00",
            "2020-01-12 12:14:59",
            1,
            15,
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
        );
        let c2 = Candle::new(
            0,
            "2020-01-12 12:15:00",
            "2020-01-12 12:29:59",
            1,
            15,
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
        );

        let d1 = c1.open_time;
        let d2 = c2.open_time;

        let d15m = Duration::minutes(15);
        assert_eq!(d2 - d1, d15m);

        assert_eq!(_candles_sorted_ok(&[&c1, &c2]), true);
        assert_eq!(_candles_sorted_ok(&[&c2, &c1]), false);
        assert_eq!(_candles_sorted_ok(&[&c1, &c1]), false);
        assert_eq!(_candles_sorted_ok(&[&c2, &c2]), false);

        assert_eq!(inconsistent_candles(&[&c1, &c2], &d15m).len(), 0);
        assert_eq!(inconsistent_candles(&[&c2, &c1], &d15m).len(), 1);
        assert_eq!(inconsistent_candles(&[&c1, &c1], &d15m).len(), 1);
        assert_eq!(inconsistent_candles(&[&c2, &c2], &d15m).len(), 1);

        let c3 = Candle::new(
            0,
            "2020-11-16 01:25:00",
            "2020-11-16 01:29:59",
            1,
            15,
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
        );

        let c4 = Candle::new(
            0,
            "2020-11-20 11:15:00",
            "2020-11-20 11:29:59",
            1,
            15,
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
            fdec(100.0),
        );

        assert_eq!(inconsistent_candles(&[&c3, &c4], &d15m).len(), 1);
    }
}
