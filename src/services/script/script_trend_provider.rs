use super::script_state::ScriptState;
use super::script_state_singleton::ScriptStateSingleton;
use super::singleton_context::ContextSingleton;
use super::singleton_engine::EngineSingleton;
use super::singleton_position::PositionRegisterSingleton;
use crate::services::script::position_register::PositionRegister;
use crate::services::trader::running_script_state::TrendState;
use crate::services::trader::trade_context_provider::TradeContextProvider;
use crate::services::trader::trade_operation::TradeOperation;
use crate::services::trader::trend::trend_direction::TrendDirection;
use crate::services::trader::trend::trend_provider::TrendProvider;
use std::cmp::Ordering;

pub struct ScriptTrendProvider {}

impl ScriptTrendProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ScriptTrendProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TrendProvider for ScriptTrendProvider {
    fn trend(
        &mut self,
        position_register: &PositionRegister,
        trade_context_provider: &TradeContextProvider,
    ) -> color_eyre::Result<TrendState> {
        call_back_trend_provider(position_register, trade_context_provider)
    }
}

#[inline]
fn call_back_trend_provider(
    position_register: &PositionRegister,
    trade_context_provider: &TradeContextProvider,
) -> eyre::Result<TrendState> {
    let changed_trend = trade_context_provider.changed_trend();

    let now = trade_context_provider.now();
    let price = trade_context_provider.price();

    // Set current static trade_context_provider and position to script functions can read this
    ContextSingleton::set_current(trade_context_provider.clone());
    PositionRegisterSingleton::set_current(position_register.clone());

    ScriptStateSingleton::set_current(ScriptState {
        log: None,
        operation_opt: None,
        changed_trend,
        trend_direction: TrendDirection::None,
    });

    // Get engine to run script
    let engine_arc = EngineSingleton::current();
    let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();

    // Run script
    let _: () = engine.call_fn(&mut scope.clone(), &ast, "run", ()).unwrap();

    let singleton = ScriptStateSingleton::current();
    let script_state = singleton.script_state_opt.as_ref().unwrap();
    let trend_direction = script_state.trend_direction;

    let trade_operation_opt = script_state
        .operation_opt
        .map(|operation| TradeOperation::new(operation, now, price, script_state.log.clone()));

    Ok(TrendState {
        trend_direction,
        trade_operation_opt,
    })
}
