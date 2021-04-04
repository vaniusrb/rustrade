use std::fmt::Display;
use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Display)]
pub struct Symbol {
    id: i32,
    symbol: String,
}
