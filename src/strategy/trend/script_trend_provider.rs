use rhai::Engine;

use super::trend_provider::TrendProvider;
use crate::strategy::{trade_context_provider::TradeContextProvider, trend_enum::Trend};

type CallBackTrend = dyn Fn(&TradeContextProvider) -> anyhow::Result<Trend>;

pub struct ScriptTrendProvider {
    //vm: RootedThread,
    engine: Engine,
    // trade_context_provider: RefCell<TradeContextProvider>,
    trade_context_provider: TradeContextProvider,
}

impl ScriptTrendProvider {
    pub fn from(trade_context_provider: TradeContextProvider) -> Self {
        Self {
            engine: Engine::new(),
            //vm: new_vm(),
            // trade_context_provider: RefCell::new(trade_context_provider),
            trade_context_provider,
        }
    }

    // pub fn get_mcad(&self, a: usize, b: usize, c: usize) -> f64 {
    //     self.trade_context_provider
    //         .borrow_mut()
    //         .indicator(15, &IndicatorType::Macd(a, b, c))
    //         .unwrap()
    //         .value()
    //         .unwrap()
    // }

    // fn context_module_fn(&self, vm: &Thread) -> Result<vm::ExternModule, vm::Error> {
    //     vm::ExternModule::new(
    //         vm,
    //         record!(
    //         mcad => primitive!(3, teste),
    //         //mcad => mcad,
    //         //mcad_signal => mcad_signal),
    //         ),
    //     )
    // }

    //     fn set_call_back_trend(&self mut, call_back_trend: Box<CallBackTrend>) {
    //     }
}

fn teste(a: u32, b: u32, c: u32) {}

// https://github.com/gluon-lang/gluon/issues/873

impl<'a> TrendProvider for ScriptTrendProvider {
    fn trend(&self, trend_context_provider: &TradeContextProvider) -> anyhow::Result<Trend> {
        // add_extern_module(&self.vm, "context", context_module);

        //self.engine.register_fn("mcad", |a, b, c| self.get_mcad(a, b, c));

        //let trend = if mcad > mcad_signal { Trend::Bought } else { Trend::Sold };
        Ok(Trend::Bought)
    }
}
