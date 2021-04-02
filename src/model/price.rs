use ifmt::iwrite;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Clone, Debug, Copy)]
pub struct Price(pub Decimal);

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        iwrite!(f, "{self.0.to_string()}")
    }
}
