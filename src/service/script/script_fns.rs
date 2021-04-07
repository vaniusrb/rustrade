use super::{singleton_context::ContextSingleton, singleton_position::PositionRegisterSingleton};
use crate::service::technicals::ind_type::IndicatorType;
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

fn gain_perc() -> u32 {
    todo!()
}

fn loss_perc() -> u32 {
    todo!()
}

pub fn macd(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(
            min as i32,
            &IndicatorType::Macd(a as usize, b as usize, c as usize),
        )
        .unwrap()
        .value()
        .unwrap()
}

pub fn macd_signal(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(
            min as i32,
            &IndicatorType::MacdSignal(a as usize, b as usize, c as usize),
        )
        .unwrap()
        .value()
        .unwrap()
}

pub fn macd_divergence(min: i64, a: i64, b: i64, c: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(
            min as i32,
            &IndicatorType::MacdDivergence(a as usize, b as usize, c as usize),
        )
        .unwrap()
        .value()
        .unwrap()
}

pub fn ema(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(min as i32, &IndicatorType::Ema(a as usize))
        .unwrap()
        .value()
        .unwrap()
}

pub fn sma(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(min as i32, &IndicatorType::Sma(a as usize))
        .unwrap()
        .value()
        .unwrap()
}

pub fn rsi(min: i64, a: i64) -> f64 {
    let singleton = ContextSingleton::current();
    let trade_context_provider = singleton.trade_context_provider_opt.as_ref().unwrap();
    trade_context_provider
        .indicator(min as i32, &IndicatorType::Rsi(a as usize))
        .unwrap()
        .value()
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
