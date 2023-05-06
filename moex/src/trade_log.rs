use market_datatypes::{Price, OrderPrice};

#[derive(Debug, Clone, Copy)]
pub struct TradeLog {
    /// price the order was executed at
    /// TRADEPRICE/PRICE_DEAL
    pub price: OrderPrice,
    /// ID_DEAL/TRADENO
    pub id: i64,
}
