use ifmt::iwrite;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Clone, Debug, Copy)]
pub struct Quantity(pub Decimal);

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        iwrite!(f, "{self.0.to_string()}")
    }
}
