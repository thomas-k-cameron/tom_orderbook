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
/// 6.4.5 建値通知タグ （タグ ID ： P ）
///
/// （１） タグ内容
///
/// コンボ銘柄とコンボ銘柄が約定したときに、レグ銘柄の建値を提供する。
///
/// 潜在的なインプライド注文等、板に表示が無い状態で約定が発生した場合、価格を通知するために本タグが配信される。
///
/// （２） タグ出力タイミング
///
/// 既発注の注文が無い板で約定が発生した場合に配信される。
///
/// また、コンボ銘柄とコンボ銘柄が約定した場合、コンボ同士の約定を示す価格情報付約定通知(C タグ)と、コンボの構成レグに対して、約定価
///
/// 格を通知する建値通知タグ（P タグ）をレグの数だけ配信する。
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct LegPrice {
    pub timestamp: NaiveDateTime,
    pub combo_group_id: i64,
    pub match_id: i64,
    pub occurred_at_cross: bool,
    pub order_book_id: i64,
    pub quantity: i64,
    pub trade_price: i64,
}

impl_message! {
    name: LegPrice 'P';
    pub timestamp: NaiveDateTime,
    pub combo_group_id: i64,
    pub match_id: i64,
    pub occurred_at_cross: bool,
    pub order_book_id: i64,
    pub quantity: i64,
    pub trade_price: i64,
}

impl TryFrom<&str> for LegPrice {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('P', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).unwrap();
        let match_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let combo_group_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let _side = iter.next();
        let quantity = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let order_book_id = extract_value_and_parse(iter.next().ok_or(())?).unwrap();
        let trade_price = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        iter.next();
        iter.next();
        iter.next();
        let _occurred_at_cross: String = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let occurred_at_cross = _occurred_at_cross.as_str() == "Y";

        Ok(Self {
            timestamp,
            combo_group_id,
            match_id,
            occurred_at_cross,
            order_book_id,
            quantity,
            trade_price,
        })
    }
}
