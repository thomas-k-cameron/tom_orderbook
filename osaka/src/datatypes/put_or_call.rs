use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone, Copy)]
pub enum PutOrCall {
    Put = 1,
    Call = 2,
    Combo = 0,
}

impl FromStr for PutOrCall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" | "CAL" => PutOrCall::Call,
            "2" | "PUT" => PutOrCall::Put,
            "0" => PutOrCall::Combo,
            _ => return Err(()),
        })
    }
}

impl TryFrom<i8> for PutOrCall {
    type Error = ();

    fn try_from(int: i8) -> Result<Self, Self::Error> {
        match int {
            1 => Ok(PutOrCall::Call),
            2 => Ok(PutOrCall::Put),
            0 => Ok(PutOrCall::Combo),
            _ => Err(()),
        }
    }
}

impl PutOrCall {
    pub fn is_call(&self) -> bool {
        matches!(self, PutOrCall::Call)
    }

    pub fn is_put(&self) -> bool {
        matches!(self, PutOrCall::Put)
    }

    pub fn is_combo(&self) -> bool {
        matches!(self, PutOrCall::Combo)
    }
}
