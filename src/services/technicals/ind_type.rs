#[derive(PartialEq, Eq, Hash)]
pub enum IndicatorType {
    Macd(usize, usize, usize),
    MacdSignal(usize, usize, usize),
    MacdDivergence(usize, usize, usize),
    Ema(usize),
    Sma(usize),
    Rsi(usize),
    MinMax(usize),
    //TopBottom(usize),
}

impl IndicatorType {
    pub fn period(&self) -> i32 {
        match self {
            IndicatorType::Macd(period, _, _) => *period as i32,
            IndicatorType::MacdSignal(period, _, _) => *period as i32,
            IndicatorType::MacdDivergence(period, _, _) => *period as i32,
            IndicatorType::Ema(period) => *period as i32,
            IndicatorType::Sma(period) => *period as i32,
            IndicatorType::Rsi(period) => *period as i32,
            IndicatorType::MinMax(period) => *period as i32,
        }
    }
}
