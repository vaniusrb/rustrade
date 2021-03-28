use super::trend_enum::Trend;

pub struct OrderExecutor {
    previous_trend: Option<Trend>,
}

impl OrderExecutor {
    pub fn new() -> Self {
        Self { previous_trend: None }
    }
}
