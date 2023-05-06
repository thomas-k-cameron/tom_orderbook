use serde::{
    Deserialize,
    Serialize,
};

use crate::Side;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Serialize, Deserialize, Clone, Copy)]
pub struct LegSide(Side);

impl TryFrom<&str> for LegSide {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "66" => Ok(LegSide(Side::Buy)),
            "67" => Ok(LegSide(Side::Sell)),
            _ => Err(value.to_string()),
        }
    }
}

impl LegSide {
    pub const BUY: Self = LegSide(Side::Buy);
    pub const SELL: Self = LegSide(Side::Sell);

    #[inline]
    pub fn inner(&self) -> Side {
        self.0
    }
}

#[test]
fn test_leg_side() {
    assert_eq!(LegSide::try_from("66"), Ok(LegSide::BUY));
    assert_eq!(LegSide::try_from("67"), Ok(LegSide::SELL));

    assert_eq!(LegSide::try_from("2134"), Err("2134".to_string()));
    assert_eq!(LegSide::try_from("B"), Err("B".to_string()));

    assert_eq!(LegSide::BUY.inner(), Side::Buy);
    assert_eq!(LegSide::SELL.inner(), Side::Sell);
}
