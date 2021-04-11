use super::trend::trend_direction::TrendDirection;
use crate::model::operation::Operation;

pub struct TrendState {
    pub trend_direction: TrendDirection,
    pub operation_opt: Option<Operation>,
}
