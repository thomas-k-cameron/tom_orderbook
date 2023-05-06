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
/// 6.4.3 価格情報付約定通知タグ （タグ ID ： C ）
///
/// （１） タグ内容
///
/// 価格情報付きの約定通知を提供する。ザラバ中において、既発注の注文サイドをセット。（売り注文が存在し、買い注文により約定した場合は、
///
/// 売り注文に係る注文受付番号をセットする。）
///
/// （２） タグ出力タイミング
///
/// コンボ銘柄とコンボ銘柄が約定した場合と、板寄せ約定時に配信する。また、既発注の注文と別の値段で約定した場合に配信する。
///
/// ・コンボ銘柄とコンボ銘柄が約定した場合、価格情報付約定通知(C タグ)と、コンボの構成レグの約定価格を通知する建値通知タグ（P タグ）
///
/// をレグの数だけ配信する。
///
/// 板寄せで複数の注文が約定した場合は既発注の注文サイドに関係なく、約定した売注文、及び買注文すべてに対して C タグが配信される。
///
/// Occured At Cross
/// 板寄せ約定判別フラグ
/// コード 項目設定値説明
/// - Y 板寄約定
/// - N ザラバ約定
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct ExecutionWithPriceInfo {
    pub timestamp: NaiveDateTime,
    pub combo_group_id: i64,
    pub executed_quantity: i64,
    pub match_id: i64,
    /// 板寄せ約定判別フラグ
    /// コード 項目設定値説明
    /// - Y 板寄約定
    /// - N ザラバ約定
    /// ここでは、Y出合った場合にtrue, Nで会った場合にfalseとしている。
    ///
    /// おそらく、Y == YES, N == NOと思われるため。知らんけど
    pub occurred_at_cross: bool,
    pub order_book_id: i64,
    pub order_id: i64,
    pub side: Side,
    pub trade_price: i64,
}

impl_message! {
    name: ExecutionWithPriceInfo 'C';
    pub timestamp: NaiveDateTime,
    pub combo_group_id: i64,
    pub executed_quantity: i64,
    pub match_id: i64,
    pub occurred_at_cross: bool,
    pub order_book_id: i64,
    pub order_id: i64,
    pub side: Side,
    pub trade_price: i64,
}

impl TryFrom<&str> for ExecutionWithPriceInfo {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('C', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;
        let order_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let order_book_id = extract_value_and_parse(iter.next().ok_or(())?).ok_or(())?;

        let side = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        let executed_quantity = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let match_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let combo_group_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        let _reserved = iter.next();
        let _reserved = iter.next();

        let trade_price = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let occurred_at_cross: String = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        Ok(Self {
            timestamp,
            combo_group_id,
            executed_quantity,
            match_id,
            occurred_at_cross: occurred_at_cross.as_str() == "Y",
            order_book_id,
            order_id,
            side,
            trade_price,
        })
    }
}
