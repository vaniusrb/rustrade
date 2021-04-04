use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct SymbolMinutes {
    pub symbol: i32,
    pub minutes: i32,
}

impl SymbolMinutes {
    pub fn new(symbol: &i32, minutes: &i32) -> Self {
        Self {
            symbol: *symbol,
            minutes: *minutes,
        }
    }
}
