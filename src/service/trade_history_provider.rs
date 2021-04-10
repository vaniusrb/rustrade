pub struct TradeHistoryProvider {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::symbol_minutes::SymbolMinutes,
        repository::{repository_candle::RepositoryCandle, repository_factory::create_pool},
    };
    use chrono::{Duration, Utc};

    #[test]
    fn trade_history_test() {
        let pool = create_pool(log::LevelFilter::Debug).unwrap();
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(30);
        let repo = RepositoryCandle::new(pool);
        let symbol_minutes = SymbolMinutes::new(1, 15);

        let candles = repo
            .candles_by_time(&symbol_minutes, &start_time, &end_time)
            .unwrap_or_default();

        let repository_symbol = RepositorySymbol::new(pool.clone());

        let exchange: Exchange = Exchange::new(repository_symbol, Level::Debug)?;
    }
}
