use super::indicator::Indicator;
use super::technical::{TechnicalDefinition, TechnicalIndicators};
use super::top_bottom::TopBottom;
use super::top_bottom::TopBottomType;
use crate::config::definition::TacDefinition;
use crate::model::candle::Candle;
use std::collections::HashSet;

pub const IND_TOP_BOTTOM: &str = "topbottom";

pub const TEC_TOP_BOTTOM: &str = "topbottom";

#[derive(Clone)]
pub struct TopBottomTec {
    top_bottoms: Vec<TopBottom>,
}

impl TechnicalDefinition for TopBottomTec {
    fn definition() -> TacDefinition {
        TacDefinition::new("topbottom", &["topbottom"])
    }
}

impl TechnicalIndicators for TopBottomTec {
    fn get_indicator(&self, name: &str) -> Option<&dyn Indicator> {
        todo!()
    }

    fn main_indicator(&self) -> &dyn Indicator {
        todo!()
    }

    fn name(&self) -> String {
        TEC_TOP_BOTTOM.to_string()
    }
}

impl TopBottomTec {
    pub fn new(candles: &[Candle], period: usize, neighbors: usize) -> Self {
        let start = (candles.len() - period).max(0);
        let candles = &candles[start..candles.len()];

        let mut top_bottoms = Vec::new();

        for i in 0..candles.len() - (neighbors * 2 + 1) {
            let candle = &candles[i + neighbors];
            let l_min = candles[i..i + neighbors]
                .iter()
                .map(|c| c.low)
                .min()
                .unwrap_or(candle.low);
            let l_max = candles[i..i + neighbors]
                .iter()
                .map(|c| c.high)
                .max()
                .unwrap_or(candle.high);
            let r_min = candles[i + neighbors + 1..i + (neighbors * 2 + 1)]
                .iter()
                .map(|c| c.low)
                .min()
                .unwrap_or(candle.low);
            let r_max = candles[i + neighbors + 1..i + (neighbors * 2 + 1)]
                .iter()
                .map(|c| c.high)
                .max()
                .unwrap_or(candle.high);
            if candle.low < l_min && candle.low < r_min {
                top_bottoms.push(TopBottom::new(
                    TopBottomType::Top,
                    candle.close_time,
                    candle.low,
                ));
            }
            if candle.high > l_max && candle.high > r_max {
                top_bottoms.push(TopBottom::new(
                    TopBottomType::Bottom,
                    candle.close_time,
                    candle.high,
                ));
            }
        }
        normalize_top_bottoms(&mut top_bottoms);

        TopBottomTec { top_bottoms }
    }

    pub fn top_bottoms(&self) -> eyre::Result<Vec<TopBottom>> {
        Ok(self.top_bottoms.clone())
    }

    pub fn last_n_bottom(&self, last: u32) -> Option<TopBottom> {
        let result = self
            .top_bottoms
            .iter()
            .filter(|tb| tb.type_p == TopBottomType::Bottom)
            .collect::<Vec<_>>();
        result
            .get(result.len() - last as usize - 1)
            .map(|tb| *tb)
            .cloned()
    }

    pub fn last_n_top(&self, last: u32) -> Option<TopBottom> {
        let result = self
            .top_bottoms
            .iter()
            .filter(|tb| tb.type_p == TopBottomType::Top)
            .collect::<Vec<_>>();
        result
            .get(result.len() - last as usize - 1)
            .map(|tb| *tb)
            .cloned()
    }
}

fn normalize_top_bottoms(top_bottoms: &mut Vec<TopBottom>) {
    if top_bottoms.is_empty() {
        return;
    }

    let mut delete = HashSet::new();
    let mut reverse = top_bottoms.clone();
    reverse.reverse();

    let mut top_bottoms_iter = reverse.iter();

    let mut previous = top_bottoms_iter.next().unwrap();
    loop {
        match top_bottoms_iter.next() {
            None => break,
            Some(current) => {
                if current.type_p == previous.type_p {
                    if current.type_p == TopBottomType::Top {
                        delete.insert(max_price(previous, current));
                    } else {
                        delete.insert(min_price(previous, current));
                    }
                }
                previous = current;
            }
        }
    }

    top_bottoms.retain(|p| delete.get(p).is_none());
}

fn max_price<'a>(previous: &'a TopBottom, current: &'a TopBottom) -> &'a TopBottom {
    if previous.price > current.price {
        previous
    } else {
        current
    }
}

fn min_price<'a>(previous: &'a TopBottom, current: &'a TopBottom) -> &'a TopBottom {
    if previous.price < current.price {
        previous
    } else {
        current
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::model::candle::Candle;
    use crate::services::provider::candles_provider::CandlesProvider;
    use crate::services::provider::candles_provider::CandlesProviderVec;
    use crate::utils::date_utils::str_to_datetime;
    use ifmt::iprintln;
    use rust_decimal_macros::dec;

    #[test]
    fn topbottom_test() -> color_eyre::eyre::Result<()> {
        let c1 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 12:00:00"),
            close_time: str_to_datetime("2020-01-12 12:14:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(100.0),
            volume: dec!(100.0),
        };

        let c2 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 12:15:00"),
            close_time: str_to_datetime("2020-01-12 12:29:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(102.0),
            low: dec!(102.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c3 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 12:30:00"),
            close_time: str_to_datetime("2020-01-12 12:44:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(103.0),
            low: dec!(103.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c4 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 12:45:00"),
            close_time: str_to_datetime("2020-01-12 12:59:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(104.0),
            low: dec!(104.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c5 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 13:00:00"),
            close_time: str_to_datetime("2020-01-12 13:14:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(105.0),
            low: dec!(105.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c6 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 13:15:00"),
            close_time: str_to_datetime("2020-01-12 13:29:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(106.0),
            low: dec!(106.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c7 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 13:30:00"),
            close_time: str_to_datetime("2020-01-12 13:44:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(107.0),
            low: dec!(107.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c8 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 13:45:00"),
            close_time: str_to_datetime("2020-01-12 13:59:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(108.0),
            low: dec!(108.0),
            close: dec!(108.0),
            volume: dec!(100.0),
        };

        let c9 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 14:00:00"),
            close_time: str_to_datetime("2020-01-12 14:14:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(107.0),
            low: dec!(107.0),
            close: dec!(107.0),
            volume: dec!(100.0),
        };

        let c10 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 14:15:00"),
            close_time: str_to_datetime("2020-01-12 14:29:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(106.0),
            low: dec!(106.0),
            close: dec!(106.0),
            volume: dec!(100.0),
        };

        let c11 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 14:30:00"),
            close_time: str_to_datetime("2020-01-12 14:44:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(105.0),
            low: dec!(105.0),
            close: dec!(105.0),
            volume: dec!(100.0),
        };

        let c12 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 14:45:00"),
            close_time: str_to_datetime("2020-01-12 14:59:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(104.0),
            low: dec!(104.0),
            close: dec!(104.0),
            volume: dec!(100.0),
        };

        let c13 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 15:00:00"),
            close_time: str_to_datetime("2020-01-12 15:14:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(103.0),
            low: dec!(103.0),
            close: dec!(103.0),
            volume: dec!(100.0),
        };

        let c14 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 15:15:00"),
            close_time: str_to_datetime("2020-01-12 15:29:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(102.0),
            low: dec!(102.0),
            close: dec!(102.0),
            volume: dec!(100.0),
        };

        let c15 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 15:30:00"),
            close_time: str_to_datetime("2020-01-12 15:44:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(101.0),
            low: dec!(101.0),
            close: dec!(101.0),
            volume: dec!(100.0),
        };

        let c16 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 15:45:00"),
            close_time: str_to_datetime("2020-01-12 15:59:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(100.0),
            low: dec!(100.0),
            close: dec!(100.0),
            volume: dec!(100.0),
        };

        let c17 = Candle {
            id: 0,
            open_time: str_to_datetime("2020-01-12 16:00:00"),
            close_time: str_to_datetime("2020-01-12 16:14:59"),
            symbol: 1, // symbol_from_string("BTCUSDT"),
            minutes: 15,
            open: dec!(100.0),
            high: dec!(99.0),
            low: dec!(99.0),
            close: dec!(99.0),
            volume: dec!(100.0),
        };

        let candles = [
            &c1, &c2, &c3, &c4, &c5, &c6, &c7, &c8, &c9, &c10, &c11, &c12, &c13, &c14, &c15, &c16,
            &c17,
        ];
        let candles_vec = candles.iter().cloned().cloned().collect::<Vec<_>>();
        let candles_provider_vec = CandlesProviderVec::new(&candles_vec, 17);

        let mut candles_provider = Box::new(candles_provider_vec);
        let candles = candles_provider.candles()?;

        let mut topbottom_tac = TopBottomTec::new(&candles, candles.len(), 7);

        let top_bottoms = topbottom_tac.top_bottoms().unwrap();

        iprintln!("{top_bottoms.len()}");
        for topbottom in top_bottoms.iter() {
            iprintln!("{topbottom:?}");
        }
        Ok(())
    }
}
