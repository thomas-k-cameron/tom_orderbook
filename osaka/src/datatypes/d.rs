// automatically generated
use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::util::{
    extract_datetime,
    extract_value_and_parse,
};
use crate::{
    tag_guard,
    Side,
};

///
/// 6.4.4 削除注文タグ （タグ ID ： D ）
///
/// （１） タグ内容
///
/// 注文が削除され、板上に存在いなくなった場合に提供する。
///
/// （２） タグ出力タイミング
///
/// 通常、注文が削除となった場合に提供する。
///
/// 注文訂正時は、削除注文タグによる取消し後、訂正後の注文に係る新規注文タグが出力される。
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct DeleteOrder {
    pub timestamp: NaiveDateTime,
    pub order_book_id: i64,
    pub order_id: i64,
    pub side: Side,
}

impl_message! {
    name: DeleteOrder 'D';
    pub timestamp: NaiveDateTime,
    pub order_book_id: i64,
    pub order_id: i64,
    pub side: Side,
}

impl TryFrom<&str> for DeleteOrder {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('D', s);
        let mut iter = s.split(",").skip(1);

        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;

        let order_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let order_book_id = extract_value_and_parse(iter.next().ok_or(())?).ok_or(())?;

        let side = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        Ok(Self {
            timestamp,
            order_book_id,
            order_id,
            side,
        })
    }
}
