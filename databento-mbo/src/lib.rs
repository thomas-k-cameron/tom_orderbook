use std::ffi::c_char;

use dbn::record::MboMsg;
use market_datatypes::Side;
use tom_orderbook::{MakerOrder, OrderBook, UniqueOrderId};

pub fn into_maker_order(value: &MboMsg) -> Option<MakerOrder> {
    Some(MakerOrder {
        price: value.price.into(),
        side: {
            match value.side as u8 as char {
                'A' => Side::Sell,
                'B' => Side::Buy,
                'N' => {
                    todo!()
                }
                _ => return None,
            }
        },
        qty: value.size as i64,
        id: value.order_id,
    })
}
