use super::trade_operation::TradeOperation;
use super::trend::trend_direction::TrendDirection;

pub struct TrendState {
    pub trend_direction: TrendDirection,
    pub trade_operation_opt: Option<TradeOperation>,
}
