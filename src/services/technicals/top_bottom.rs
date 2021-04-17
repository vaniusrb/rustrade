use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum TopBottomType {
    Top,
    Bottom,
}

impl std::fmt::Display for TopBottomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if *self == TopBottomType::Top {
            "Low"
        } else {
            "High"
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopBottom {
    pub close_time: DateTime<Utc>,
    pub price: Decimal,
    pub type_p: TopBottomType,
}

impl TopBottom {
    pub fn new(type_p: TopBottomType, close_time: DateTime<Utc>, price: Decimal) -> Self {
        Self {
            close_time,
            price,
            type_p,
        }
    }
}

impl PartialOrd for TopBottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.close_time.cmp(&other.close_time))
    }
}

impl Ord for TopBottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.close_time.cmp(&other.close_time)
    }
}
