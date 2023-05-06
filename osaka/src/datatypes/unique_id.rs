use std::convert::Infallible;

use serde::Serialize;

use crate::{
    AddOrder,
    DeleteOrder,
    Side,
};

/// Orders are uniquely identified by its order_id, order_book_id and it's side (Buy Or Sell)
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Clone, Copy, Default)]
pub struct UniqueId {
    pub order_book_id: i64,
    pub order_id: i64,
    pub side: Side,
}

impl UniqueId {
    pub fn from_add_order(put: &AddOrder) -> Self {
        Self {
            order_book_id: put.order_book_id,
            order_id: put.order_id,
            side: put.side,
        }
    }

    pub fn from_delete_order(del: &DeleteOrder) -> Self {
        Self {
            order_book_id: del.order_book_id,
            order_id: del.order_id,
            side: del.side,
        }
    }
}

impl<'a> TryFrom<&'a AddOrder> for UniqueId {
    type Error = Infallible;

    fn try_from(a: &'a AddOrder) -> Result<Self, Infallible> {
        Ok(UniqueId::from_add_order(&a))
    }
}

impl<'a> TryFrom<&'a DeleteOrder> for UniqueId {
    type Error = Infallible;

    fn try_from(a: &'a DeleteOrder) -> Result<Self, Infallible> {
        Ok(UniqueId::from_delete_order(&a))
    }
}


impl ToString for UniqueId {
    fn to_string(&self) -> String {
        format!("{}-{}-{:?}", self.order_book_id, self.order_id, self.side)
    }
}