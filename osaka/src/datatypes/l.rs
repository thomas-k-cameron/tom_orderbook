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
/// 6.3.4 呼値単位情報タグ （タグ ID ： L ）
///
/// （１） タグ内容
///
/// 取引銘柄の呼値の単位を提供する。
///
/// （２） タグ出力タイミング
///
/// オンライン開始後、一定時間経過後に提供する。
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct TickSize {
    pub timestamp: NaiveDateTime,
    pub order_book_id: i64,
    pub price_from: i64,
    pub price_to: i64,
    pub tick_size: i64,
}

impl_message! {
    name: TickSize 'L';
    pub timestamp: NaiveDateTime,
    pub order_book_id: i64,
    pub price_from: i64,
    pub price_to: i64,
    pub tick_size: i64,
}

impl TryFrom<&str> for TickSize {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('L', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;
        let order_book_id = extract_value_and_parse(iter.next().ok_or(())?).ok_or(())?;
        let tick_size = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let price_from = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let price_to = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        Ok(Self {
            timestamp,
            order_book_id,
            price_from,
            price_to,
            tick_size,
        })
    }
}
