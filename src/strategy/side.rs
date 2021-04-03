use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, Display, Copy)]
pub enum Side {
    Bought,
    Sold,
}

impl Side {}
