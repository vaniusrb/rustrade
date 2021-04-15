use super::script_state::ScriptState;
use super::script_state_singleton::ScriptStateSingleton;
use super::singleton_context::ContextSingleton;
use super::singleton_engine::EngineSingleton;
use super::singleton_position::PositionRegisterSingleton;
use crate::model::operation::Operation;
use crate::model::quantity::Quantity;
use crate::services::script::position_register::PositionRegister;
use crate::services::trader::running_script_state::TrendState;
use crate::services::trader::trade_context_provider::TradeContextProvider;
use crate::services::trader::trade_operation::TradeOperation;
use crate::services::trader::trend::trend_direction::TrendDirection;
use crate::services::trader::trend::trend_provider::TrendProvider;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
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

    ScriptStateSingleton::set_current(ScriptState { log: None });

    // Get engine to run script
    let engine_arc = EngineSingleton::current();
    let (engine, scope, ast) = &engine_arc.engine_scope.as_ref().unwrap();

    // Retrieving trend direction
    let trend_direction = {
        let trend: i64 = engine
            .call_fn(&mut scope.clone(), &ast, "trend", ())
            .unwrap();
        match trend.cmp(&0) {
            Ordering::Greater => TrendDirection::Buy,
            Ordering::Less => TrendDirection::Sell,
            Ordering::Equal => TrendDirection::None,
        }
    };

    // Retrieving quantity to buy or sell
    let operation_opt = {
        let quantity: f64 = if let Some(trend_direction) = changed_trend {
            let trend = match trend_direction {
                TrendDirection::Buy => 1,
                TrendDirection::Sell => -1,
                TrendDirection::None => 0,
            };
            engine
                .call_fn(&mut scope.clone(), &ast, "change_trend", (trend as i64,))
                .unwrap()
        } else {
            engine.call_fn(&mut scope.clone(), &ast, "run", ()).unwrap()
        };
        quantity_to_operation_opt(quantity)
    };

    let trade_operation_opt = operation_opt.map(|operation| {
        let singleton = ScriptStateSingleton::current();
        let trade_context_provider = singleton.script_state_opt.as_ref().unwrap();
        let description = trade_context_provider.log.clone();
        TradeOperation::new(operation, now, price, description)
    });

    Ok(TrendState {
        trend_direction,
        trade_operation_opt,
    })
}

fn quantity_to_operation_opt(quantity: f64) -> Option<Operation> {
    if quantity > 0. {
        Some(Operation::Buy(Quantity(
            Decimal::from_f64(quantity).unwrap(),
        )))
    } else if quantity < 0. {
        Some(Operation::Sell(Quantity(
            Decimal::from_f64(quantity * -1.).unwrap(),
        )))
    } else {
        None
    }
}
