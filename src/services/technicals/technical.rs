use super::indicator::Indicator;
use super::serie_indicator::SerieIndicator;
use crate::config::definition::TacDefinition;
use std::collections::HashMap;

pub trait TechnicalDefinition {
    fn definition() -> TacDefinition;
}

pub trait TechnicalIndicators {
    fn get_indicator(&self, name: &str) -> Option<&dyn Indicator> {
        self.indicators().get(name).map(|s| s as &dyn Indicator)
    }

    fn indicators(&self) -> &HashMap<String, SerieIndicator>;
    fn main_indicator(&self) -> &dyn Indicator;
    fn name(&self) -> String;
}

pub trait TecSerieIndicators {
    fn serie_indicators(&self) -> &HashMap<String, SerieIndicator>;
    fn name(&self) -> String;
}
