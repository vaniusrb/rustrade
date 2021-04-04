#[derive(Clone, Copy)]
pub struct Position {
    pub id: i32,
    pub balance_asset: Decimal,
    pub balance_fiat: Decimal,
    pub price: Price,
    pub real_balance_fiat: Decimal,
}
