use std::convert::Infallible;



#[derive(Debug, Clone, Copy)]
pub enum OrderPrice<P = f64> 
    where
        P: Copy + Clone,
{
    Market,
    Limit(P),
}

impl From<f64> for OrderPrice {
    fn from(value: f64) -> Self {
        if value.is_normal() {
            OrderPrice::Limit(value)
        } else {
            OrderPrice::Market
        }
    }
}

impl TryFrom<OrderPrice> for f64 {
    type Error = Infallible;
    fn try_from(value: OrderPrice) -> Result<Self, Self::Error> {
        match value {
            OrderPrice::Market => Ok(f64::INFINITY),
            OrderPrice::Limit(p) => Ok(p),
        }
    }
}


