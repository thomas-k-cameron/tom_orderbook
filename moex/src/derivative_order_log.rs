use std::convert::Infallible;

use tom_orderbook::{TomsOrder, Order, UniqueOrderId};

pub use crate::crate_prelude::*;

#[derive(Debug)]
pub struct DerivativeOrderLog {
    /// time/moment
    pub timestamp: NaiveDateTime,
    /// BUYSELL/TYPE
    pub side: Side,
    /// ORDERNO | ID
    pub id: u64,
    pub action: Action,
    pub price: OrderPrice,
    pub volume: i64,
    pub name: String,
    pub derivative_type: DerivativeType,
}

impl DerivativeOrderLog {
    // #SYMBOL,SYSTEM,TYPE,MOMENT,ID,ACTION,PRICE,VOLUME,ID_DEAL,PRICE_DEAL
    pub fn new(s: &str) -> Option<Self> {
        let timestamp_fmt = "%Y%m%d%H%M%S%f";
        let mut iter = s.split(",");
        // name of the variables matches the `field name` written on the specification
        let symbol = iter.next()?;
        let system: DerivativeType = iter.next()?.try_into().ok()?;
        let side = match iter.next()? {
            "B" => Side::Buy,
            "S" => Side::Sell,
            _ => return None,
        };
        let timestamp = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
        let id = iter.next()?.parse().ok()?;
        let action_byte = iter.next()?;
        let price = {
            let decimal = iter.next()?.parse::<f64>();
            if let Ok(n) = decimal {
                OrderPrice::Limit(n)
            } else {
                OrderPrice::Market
            }
        };
        let volume = iter.next()?.parse::<i64>().ok()?;
        let action = match iter.next()? {
            "" if action_byte == "0" => Action::Cancel,
            "" if action_byte == "1" => Action::Add,
            trade_id => {
                let price = iter.next()?.parse::<f64>().ok()?.into();
                let id = trade_id.parse::<i64>().ok()?;
                Action::Trade(TradeLog { price, id })
            }
        };

        Some(DerivativeOrderLog {
            name: symbol.to_string(),
            derivative_type: system,
            side,
            action,
            price,
            volume,
            timestamp,
            id,
        })
    }
}

impl TryFrom<DerivativeOrderLog> for TomsOrder<OrderPrice, i64> {
    type Error = Infallible;
    fn try_from(value: DerivativeOrderLog) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            price: value.price,
            qty: value.volume,
            side: value.side,
        })
    }
}

impl UniqueOrderId for DerivativeOrderLog {
    fn unique_order_id(&self) -> u64 {
        self.id
    }
}

#[cfg(test)]
mod test {
    use crate::DerivativeOrderLog;

    #[test]
    fn opt_order_log() {
        let opts = DerivativeOrderLog::new(
            "Si73750BC2,C,B,20220131185256610,1892947028292403201,1,1.00000,1,,",
        );
        assert!(opts.is_some());
        let opts = DerivativeOrderLog::new(
            "Si77500BN2A,P,B,20220131190608307,1892947028292437532,2,213.00000,1,1892947028292405803,213.00000"
        );
        assert!(opts.is_some());
        println!("{:#?}", opts);
    }
}
