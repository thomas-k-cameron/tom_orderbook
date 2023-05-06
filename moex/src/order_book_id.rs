pub use crate::crate_prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Default)]
pub struct OrderBookId {
    pub name: String,
    pub asset_class: AssetClass,
}