// automatically generated

use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::tag_guard;
use crate::util::{
    extract_datetime,
    extract_value_and_parse,
};

///
/// 6.4.6 EP タグ （タグ ID ： Z ）
///
/// （１） タグ内容
///
/// 注文受付時間帯の EP 値段を提供する。
///
/// （２） タグ出力タイミング
///
/// 注文受付中、DCB 中、取引停止中等の注文受付時間帯で、タグの内容(予備項目も含む)が更新されるたびに提供される。
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct EquilibriumPrice {
    pub timestamp: NaiveDateTime,
    pub ask_qty_at_ep: i64,
    pub bid_qty_at_ep: i64,
    pub ep: i64,
    pub order_book_id: i64,
}

impl_message! {
    name: EquilibriumPrice 'Z';
    pub timestamp: NaiveDateTime,
    pub ask_qty_at_ep: i64,
    pub bid_qty_at_ep: i64,
    pub ep: i64,
    pub order_book_id: i64,
}

impl TryFrom<&str> for EquilibriumPrice {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('Z', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;
        let order_book_id = extract_value_and_parse(iter.next().ok_or(())?).ok_or(())?;
        let bid_qty_at_ep = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let ask_qty_at_ep = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let ep = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        Ok(Self {
            timestamp,
            ask_qty_at_ep,
            bid_qty_at_ep,
            ep,
            order_book_id,
        })
    }
}
