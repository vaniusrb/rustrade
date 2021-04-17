use super::indicator::Indicator;
use super::serie::Serie;
use eyre::eyre;

// TODO transform Indicator into a trait
// Rename this something like "SerieIndicator"
// Create other like SingleIndicator (to store a single value f64)
// Max and Min will be these type

#[derive(Clone)]
pub struct SerieIndicator {
    pub name: String,
    pub series: Vec<Serie>,
}

impl SerieIndicator {
    pub fn from(name: &str, series: Vec<Serie>) -> Self {
        SerieIndicator {
            name: name.into(),
            series,
        }
    }
}

impl Indicator for SerieIndicator {
    fn min_max(&self) -> (f64, f64) {
        let max = self.series.iter().fold(0f64, |acc, t| acc.max(t.value));
        let min = self.series.iter().fold(max, |acc, t| acc.min(t.value));
        (min, max)
    }

    fn value(&self) -> eyre::Result<f64> {
        Ok(self
            .series
            .last()
            .ok_or_else(|| eyre!("No last candle!"))?
            .value)
    }
}
