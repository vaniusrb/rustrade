use crate::services::technicals::technical::TechnicalDefinition;
use crate::services::technicals::top_bottom_tec::TopBottomTec;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TacDefinition {
    pub name: String,
    pub indicators: HashSet<String>,
}

impl TacDefinition {
    pub fn new(name: &str, indicators: &[&str]) -> Self {
        TacDefinition {
            name: name.into(),
            indicators: indicators.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigDefinition {
    tacs: Vec<TacDefinition>,
    minutes: Vec<u32>,
    symbol: Vec<String>,
    period_start: String,
    period_end: String,
}

impl ConfigDefinition {
    pub fn new() -> Self {
        ConfigDefinition {
            tacs: vec![TopBottomTec::definition()],
            minutes: vec![5u32, 15u32, 30u32, 60u32],
            symbol: vec!["BTCUSDT".to_string()],
            period_start: "2020-06-01 00:00:00".to_string(),
            period_end: "2020-11-30 00:00:00".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for ConfigDefinition {
    fn default() -> Self {
        Self::new()
    }
}
