use market_datatypes::{OrderId, Price, Side};
use std::collections::{HashMap, VecDeque};

mod orderbook;
pub use orderbook::OrderBook;

mod interface;
pub use crate::interface::*;
mod price_level;
use price_level::*;

mod runtime;

pub struct PriceQty {
    pub price: i64,
    pub qty: i64,
}

impl<O: Order> From<&O> for PriceQty {
    fn from(value: &O) -> Self {
        Self {
            price: value.price(),
            qty: value.qty(),
        }
    }
}

struct TickResult<'a> {
    add: &'a [PriceQty],
    replaced: &'a [PriceQty],
}
