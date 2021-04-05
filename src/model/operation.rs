use crate::model::quantity::Quantity;

use super::side::Side;

#[derive(Debug, Clone, PartialEq, Copy)]
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
