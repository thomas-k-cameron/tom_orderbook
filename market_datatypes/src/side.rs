use serde::{Deserialize, Serialize};
use std::{str::FromStr};

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

impl TryFrom<&str> for Side {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        Side::from_str(s)
    }
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Buy" => Ok(Side::Buy),
            "Sell" => Ok(Side::Sell),
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