use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Serialize, Deserialize, Clone, Copy)]
pub enum Side {
    Buy = 1,
    Sell = -1,
}

impl Default for Side {
    fn default() -> Self {
        Self::Buy
    }
}

impl TryFrom<char> for Side {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            'B' => Ok(Side::Buy),
            'S' => Ok(Side::Sell),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Side {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        Side::from_str(s)
    }
}

/// buy == true && sell == false
impl From<bool> for Side {
    fn from(s: bool) -> Self {
        match s {
            true => Side::Buy,
            false => Side::Sell,
        }
    }
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" | "66" => Ok(Side::Buy),
            "S" | "67" => Ok(Side::Sell),
            _ => Err(()),
        }
    }
}

impl Side {
    #[inline]
    pub fn is_buy(&self) -> bool {
        self == &Self::Buy
    }

    #[inline]
    pub fn is_sell(&self) -> bool {
        !self.is_buy()
    }
}
