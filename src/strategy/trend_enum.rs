use std::fmt::Display;

use crate::model::quantity::Quantity;
#[derive(Debug, Clone, PartialEq, Display)]
pub enum Side {
    Bought,
    Sold,
}

impl Side {}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Buy(Quantity),
    Sell(Quantity),
}

impl Operation {
    pub fn to_side(&self) -> Side {
        match self {
            Operation::Buy(_) => Side::Bought,
            Operation::Sell(_) => Side::Sold,
        }
    }
}
