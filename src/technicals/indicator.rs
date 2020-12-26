use chrono::{DateTime, Utc};

use super::serie::Serie;
pub struct Indicator<'a> {
    pub name: String,
    pub series: Vec<Serie<'a>>,
}

impl<'a> Indicator<'a> {
    pub fn new(name: &str) -> Self {
        Indicator {
            name: name.into(),
            series: vec![],
        }
    }

    pub fn _push(&mut self, serie: Serie<'a>) {
        self.series.push(serie);
    }

    pub fn push_serie(&mut self, date_time: &'a DateTime<Utc>, value: f64) {
        self.series.push(Serie::new(date_time, value));
    }

    pub fn min_max(&self) -> (f64, f64) {
        let max = self.series.iter().fold(0f64, |acc, t| acc.max(t.value));
        let min = self.series.iter().fold(max, |acc, t| acc.min(t.value));
        (min, max)
    }
}
