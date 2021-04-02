use super::trend_enum::Side;

pub struct OrderExecutor {
    previous_trend: Option<Side>,
}

impl OrderExecutor {
    pub fn new() -> Self {
        Self { previous_trend: None }
    }
}
