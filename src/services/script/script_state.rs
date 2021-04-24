use crate::model::operation::Operation;
use crate::services::trading::trend::trend_direction::TrendDirection;

#[derive(Clone)]
pub struct ScriptState {
    pub log: Option<String>,
    pub operation_opt: Option<Operation>,
    pub changed_trend: Option<TrendDirection>,
    pub trend_direction: TrendDirection,
}
