use super::{
    trade_operation::Trader,
    trader_register::TraderRegister,
    trend::{macd_trend_provider::MacdTrendProvider, trend_provider::TrendProvider},
};
use crate::model::position::Position;
use crate::model::price::Price;
use crate::service::candles_provider::CandlesProvider;
use crate::service::candles_provider::CandlesProviderBuffer;
use crate::service::plot_selection::plot_selection;
use crate::service::strategy::flow_register::FlowRegister;
use crate::service::technicals::ind_provider::IndicatorProvider;
use crate::Application;
use crate::{
    config::candles_selection::CandlesSelection,
    tac_plotters::{indicator_plotter::PlotterIndicatorContext, trading_plotter::TradingPlotter},
};
use eyre::eyre;
use ifmt::iformat;
use lockfree_object_pool::LinearObjectPool;
use log::info;
use rayon::prelude::*;
use rust_decimal_macros::dec;
use std::time::Instant;

#[derive(Clone)]
pub struct TraderFactory {
    candles_selection: CandlesSelection,
    candles_provider: CandlesProviderBuffer,
}

impl TraderFactory {
    pub fn from(
        candles_selection: CandlesSelection,
        candles_provider: CandlesProviderBuffer,
    ) -> Self {
        Self {
            candles_selection,
            candles_provider,
        }
    }

    pub fn create_trader(
        &self,
        trend_provider: Box<dyn TrendProvider + Send + Sync>,
        trader_register: TraderRegister,
    ) -> Trader {
        let mut candles_provider = self.candles_provider.clone();
        candles_provider.set_candles_selection(self.candles_selection.clone());
        let indicator_provider = IndicatorProvider::new();

        Trader::new(
            trend_provider,
            &self.candles_selection.symbol_minutes.symbol,
            indicator_provider,
            candles_provider,
            trader_register,
        )
    }
}

pub fn run_trader_back_test(app: &mut Application) -> eyre::Result<()> {
    let start = Instant::now();
    info!("Initializing back test...");

    let trader_factory = TraderFactory::from(
        app.selection.candles_selection.clone(),
        app.candles_provider.clone(),
    );

    app.candles_provider
        .set_candles_selection(app.selection.candles_selection.clone());
    let candles = app.candles_provider.candles()?;
    let msg = format!("Running back test... candles.len {}", candles.len());
    info!("{}", msg);

    let flow_register = FlowRegister::new();
    let price = candles
        .first()
        .ok_or_else(|| eyre!("First candle not found!"))?
        .open;

    let position = Position::from_fiat(flow_register, dec!(1000), Price(price));

    let trader_register = TraderRegister::from(position);

    let pool = LinearObjectPool::<Trader>::new(
        move || {
            let trend_provider: Box<dyn TrendProvider + Send + Sync> =
                Box::new(MacdTrendProvider::from());
            trader_factory.create_trader(trend_provider, trader_register.clone())
        },
        |_| (),
    );

    let pool_rayon = rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build()
        .unwrap();
    let trades = pool_rayon.install(|| {
        candles
            .par_iter()
            .map(|c| {
                let mut trader = pool.pull();
                trader
                    .check(/*candles_ref,*/ c.close_time, Price(c.close))
                    .unwrap();
                trader.trades()
            })
            .flatten()
            .collect::<Vec<_>>()
    });

    // TODO generating position from trades

    let trading_plotter = TradingPlotter::new(&trades);

    let plotters = vec![Box::new(trading_plotter) as Box<dyn PlotterIndicatorContext>];

    plot_selection(
        app.selection.clone(),
        app.candles_provider.clone_provider(),
        plotters,
    )?;

    info!(
        "{}",
        iformat!("Finished back test, elapsed: {start.elapsed():?}")
    );

    Ok(())
}