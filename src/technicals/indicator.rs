use super::serie::Serie;
use anyhow::anyhow;

#[derive(Clone)]
pub struct Indicator {
    pub name: String,
    pub series: Vec<Serie>,
}

impl Indicator {
    pub fn from(name: &str, series: Vec<Serie>) -> Self {
        Indicator {
            name: name.into(),
            series,
        }
    }

    pub fn min_max(&self) -> (f64, f64) {
        let max = self.series.iter().fold(0f64, |acc, t| acc.max(t.value));
        let min = self.series.iter().fold(max, |acc, t| acc.min(t.value));
        (min, max)
    }

    pub fn value(&self) -> anyhow::Result<f64> {
        Ok(self.series.last().ok_or_else(|| anyhow!("No last candle!"))?.value)
    }
}
