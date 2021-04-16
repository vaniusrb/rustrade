use super::{
    script_state_singleton::ScriptStateSingleton, singleton_context::ContextSingleton,
    singleton_position::PositionRegisterSingleton,
};
use crate::model::operation::Operation;
use crate::model::quantity::Quantity;
use crate::services::technicals::ind_type::IndicatorType;
use crate::services::trader::trend::trend_direction::TrendDirection;
use crate::utils::dec_utils::fdec;
use crate::utils::dec_utils::percent;
use colored::Colorize;
use log::info;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

pub fn price_dec() -> Decimal {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider.price().0
}

pub fn price() -> f64 {
    price_dec().to_f64().unwrap()
}

pub fn gain_perc() -> f64 {
    if !is_bought() {
        return 0.;
    }
    let singleton = PositionRegisterSingleton::current();
    let position_register = singleton.position_opt.as_ref().unwrap();
    let old = position_register.position.real_balance_fiat_r();
    let new = price_dec() * position_register.position.balance_asset_r();
    percent(&new, &old).to_f64().unwrap()
}

pub fn log(text: String) {
    info!("{} {}", "[SCRIPT]".bright_yellow(), &text.yellow());

    let singleton = ScriptStateSingleton::current();
    let trade_context_provider = singleton.script_state_opt.as_ref().unwrap();

    let mut trade_context_provider = trade_context_provider.clone();
    trade_context_provider.log = Some(text);
    ScriptStateSingleton::set_current(trade_context_provider);
}

pub fn macd(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(
            min as i32,
            &IndicatorType::Macd(a as usize, b as usize, c as usize),
        )
        .unwrap()
}

pub fn macd_signal(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(
            min as i32,
            &IndicatorType::MacdSignal(a as usize, b as usize, c as usize),
        )
        .unwrap()
}

pub fn macd_divergence(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(
            min as i32,
            &IndicatorType::MacdDivergence(a as usize, b as usize, c as usize),
        )
        .unwrap()
}

pub fn ema(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(min as i32, &IndicatorType::Ema(a as usize))
        .unwrap()
}

pub fn sma(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(min as i32, &IndicatorType::Sma(a as usize))
        .unwrap()
}

pub fn rsi(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .value(min as i32, &IndicatorType::Rsi(a as usize))
        .unwrap()
}

/// If I have more assets (equivalent value) than fiat
pub fn is_bought() -> bool {
    let singleton = PositionRegisterSingleton::current();
    let position_register = singleton.position_opt.as_ref().unwrap();
    position_register.position.balance_asset_r() * position_register.position.price
        > position_register.position.balance_fiat_r()
}

/// If I have more fiat than equivalent value of asset
pub fn is_sold() -> bool {
    !is_bought()
}

pub fn buy(quantity: f64) {
    let singleton = ScriptStateSingleton::current();
    let script_state_provider = singleton.script_state_opt.as_ref().unwrap();
    let mut script_state_provider = script_state_provider.clone();
    script_state_provider.operation_opt = Some(Operation::Buy(Quantity(fdec(quantity))));
    ScriptStateSingleton::set_current(script_state_provider);
}

pub fn sell(quantity: f64) {
    let singleton = ScriptStateSingleton::current();
    let script_state = singleton.script_state_opt.as_ref().unwrap();
    let mut script_state = script_state.clone();
    script_state.operation_opt = Some(Operation::Sell(Quantity(fdec(quantity))));
    ScriptStateSingleton::set_current(script_state);
}

pub fn change_trend_buy() -> bool {
    let singleton = ScriptStateSingleton::current();
    let script_state_provider = singleton.script_state_opt.as_ref().unwrap();
    script_state_provider.changed_trend == Some(TrendDirection::Buy)
}

pub fn change_trend_sell() -> bool {
    let singleton = ScriptStateSingleton::current();
    let script_state_provider = singleton.script_state_opt.as_ref().unwrap();
    script_state_provider.changed_trend == Some(TrendDirection::Sell)
}

pub fn set_change_trend_sell(change: bool) {
    if !change {
        return;
    }
    let singleton = ScriptStateSingleton::current();
    let script_state = singleton.script_state_opt.as_ref().unwrap();
    let mut script_state = script_state.clone();
    script_state.trend_direction = TrendDirection::Sell;
    ScriptStateSingleton::set_current(script_state);
}

pub fn set_change_trend_buy(change: bool) {
    if !change {
        return;
    }
    let singleton = ScriptStateSingleton::current();
    let script_state = singleton.script_state_opt.as_ref().unwrap();
    let mut script_state = script_state.clone();
    script_state.trend_direction = TrendDirection::Buy;
    ScriptStateSingleton::set_current(script_state);
}

pub fn balance_fiat() -> f64 {
    let singleton = PositionRegisterSingleton::current();
    let position_register = singleton.position_opt.as_ref().unwrap();
    position_register
        .position
        .balance_fiat_r()
        .to_f64()
        .unwrap()
}

pub fn balance_asset() -> f64 {
    let singleton = PositionRegisterSingleton::current();
    let position_register = singleton.position_opt.as_ref().unwrap();
    position_register
        .position
        .balance_asset_r()
        .to_f64()
        .unwrap()
}

pub fn fiat_to_asset(fiat_quantity: f64) -> f64 {
    (Decimal::from_f64(fiat_quantity).unwrap() / price_dec())
        .to_f64()
        .unwrap()
}

pub fn asset_to_fiat(asset_quantity: f64) -> f64 {
    (Decimal::from_f64(asset_quantity).unwrap() * price_dec())
        .to_f64()
        .unwrap()
}
