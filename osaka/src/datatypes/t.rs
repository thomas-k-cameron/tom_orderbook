// automatically generated

use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::tag_guard;

///
/// 6.3.1 秒タグ （タグ ID ： T ）
///
///（１） タグ内容
///
///個々のメッセージが配信される際の秒に係る情報を提供する。同一秒に複数のメッセージが配信される場合は、当該秒における最初の
///
///メッセージの前に本タグを 1 つだけ配信する。
///
///（２） タグ出力タイミング
///
///ITCH からのいずれかのメッセージの配信時。

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct SecondTag {
    pub timestamp: NaiveDateTime,
    pub second: i64,
}

impl_message! {
    name: SecondTag 'T';
    pub timestamp: NaiveDateTime,
    pub second: i64,
}

impl TryFrom<&str> for SecondTag {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('T', s);
        let mut iter = s.split(",").skip(1);
        let second = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let timestamp = NaiveDateTime::from_timestamp_opt(second, 0).unwrap();
        Ok(Self { timestamp, second })
    }
}
