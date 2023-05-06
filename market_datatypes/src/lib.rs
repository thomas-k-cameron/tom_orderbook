pub type Qty = i64;
pub type Price = i64;
pub type OrderId = u64;

mod side;
pub use side::Side;
mod order_price;
pub use order_price::OrderPrice;
