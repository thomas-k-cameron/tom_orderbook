use crate::TradeLog;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Cancel,
    Add,
    Trade(TradeLog),
}

impl Action {
    pub fn is_cancel(&self) -> bool {
        matches!(self, Self::Cancel)
    }
    pub fn is_add(&self) -> bool {
        matches!(self, Self::Add)
    }
    pub fn is_trade(&self) -> bool {
        matches!(self, Self::Trade(_))
    }
}