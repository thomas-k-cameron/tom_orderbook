// automatically generated

use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::tag_guard;
use crate::util::extract_datetime;

///
///6.3.5 システムイベント情報タグ （タグ ID ： S ）
///
///（１） タグ内容
///
///システムイベントの更新情報を提供する。
///
///（２） タグ出力タイミング
///
///システムイベントの更新のタイミングで提供する。
///
///詳細は、4.2.6(4) 章を参照のこと。
///
/// コード 項目設定値説明
/// O メッセージ送信の開始。どの営業日も、このメッセージの送信から開始。
/// C メッセージ送信の終了。どの営業日も、このメッセージの送信で終了。
/// ※1 本タグは、1 営業日で送信開始の O と送信終了の C の 2 レコードのみ配信。
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct SystemEventInfo {
    pub timestamp: NaiveDateTime,
    pub event_code: String,
}

impl_message! {
    name: SystemEventInfo 'S';
    pub timestamp: NaiveDateTime,
    pub event_code: String,
}

impl TryFrom<&str> for SystemEventInfo {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('S', s);
        let mut iter = s.split(",").skip(1);

        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;
        let event_code = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        Ok(Self {
            timestamp,
            event_code,
        })
    }
}
