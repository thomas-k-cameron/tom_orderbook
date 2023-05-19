use std::{ops::Deref, ffi};

use dbn::record::MboMsg;
use market_datatypes::Side;
use tom_orderbook::{UniqueOrderId, OrderBook};

fn main() {
}

fn into_orderbook(order_book_id: u64, msg: MboMsg) {
    let mut book = OrderBook::<DBNMboMsgWrap>::new(order_book_id);
    book.add(msg.into());
}

pub struct DBNMboMsgWrap(MboMsg);
impl Deref for DBNMboMsgWrap {
    type Target = MboMsg;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MboMsg> for DBNMboMsgWrap {
    fn from(value: MboMsg) -> Self {
        DBNMboMsgWrap(value)
    }
}

impl UniqueOrderId for DBNMboMsgWrap {
    fn unique_order_id(&self) -> u64 {
        self.order_id
    }
}

impl tom_orderbook::Order for DBNMboMsgWrap {
    fn price(&self) -> i64 {
        self.price
    }
    fn qty(&self) -> i64 {
        self.size as i64
    }
    fn side(&self) -> Side {
        match self.side as u8 as char {
            'B' => Side::Buy,
            'S' => Side::Sell,
            'N' => unimplemented!(),
            _ => unreachable!(),
        }
    }
}
