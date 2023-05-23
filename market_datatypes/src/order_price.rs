use std::{convert::Infallible, default};



#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum OrderPrice<P = i64> 
    where
        P: Copy + Clone,
{
    #[default]
    Market,
    Limit(P),
}

impl From<f64> for OrderPrice<f64> {
    fn from(value: f64) -> Self {
        if value.is_normal() {
            OrderPrice::Limit(value)
        } else {
            OrderPrice::Market
        }
    }
}

impl From<i64> for OrderPrice {
    fn from(value: i64) -> Self {
        OrderPrice::Limit(value)
    }
}

impl TryFrom<OrderPrice> for f64 {
    type Error = Infallible;
    fn try_from(value: OrderPrice) -> Result<Self, Self::Error> {
        match value {
            OrderPrice::Market => Ok(f64::INFINITY),
            OrderPrice::Limit(p) => Ok(p as f64),
        }
    }
}



impl OrderPrice<i64> {
    pub fn price_min_if_market(&self) -> i64 {
        if let OrderPrice::Limit(i) = self {
            *i
        } else {
            i64::MIN
        }
    }
}

impl OrderPrice<f64> {
    pub fn price_min_if_market(&self) -> f64 {
        if let OrderPrice::Limit(i) = self {
            *i
        } else {
            f64::MIN
        }
    }
}