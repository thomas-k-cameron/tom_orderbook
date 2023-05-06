// automatically generated
use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::util::extract_datetime;
use crate::{
    tag_guard,
    Side,
};

///
///6.3.3 銘柄情報基本タグ コンビネーション取引銘柄 （タグ ID ： M）
///
///（１） タグ内容
///
///当日取引日に取引可能なコンビネーション取引銘柄のレグ情報を提供する（取引停止中の銘柄も含む）。
///
///コンビネーション銘柄を構成するレグ毎にそれぞれレコードを分けて本タグを出力し、レグの売り・買いの別と ratio に関する情報を提供する。
///
///どのコンビネーション銘柄の対象レグかは、コンビネーション銘柄の R タグの Orderbook ID と M タグの Combination Order book ID を
///
///用いて判断する。
///
///（２） タグ出力タイミング
///
///オンライン開始後、一定時間経過後に提供する。
///
///テーラーメイドコンビネーション(TMC)が組成されたときに提供する。
///
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct CombinationProduct {
    pub timestamp: NaiveDateTime,
    pub combination_order_book_id: i64,
    pub leg_order_book_id: i64,
    pub leg_ratio: i64,
    pub leg_side: Side,
}

impl_message! {
    name: CombinationProduct 'M';
    pub timestamp: NaiveDateTime,
    pub combination_order_book_id: i64,
    pub leg_order_book_id: i64,
    pub leg_ratio: i64,
    pub leg_side: Side,
}

impl TryFrom<&str> for CombinationProduct {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('M', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;
        let combination_order_book_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let leg_order_book_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let leg_side = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let leg_ratio = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        Ok(Self {
            timestamp,
            combination_order_book_id,
            leg_order_book_id,
            leg_ratio,
            leg_side,
        })
    }
}
