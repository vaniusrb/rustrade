use super::indicator::Indicator;

pub struct ValueIndicator {
    value: f64,
}

impl ValueIndicator {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Indicator for ValueIndicator {
    fn value(&self) -> color_eyre::Result<f64> {
        Ok(self.value)
    }

    fn min_max(&self) -> (f64, f64) {
        (self.value, self.value)
    }
}
