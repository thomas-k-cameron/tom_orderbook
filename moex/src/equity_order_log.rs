pub struct EquityOrderLog {
    no: i32,
    seccode: String,
    buysell: Side,
    time: NaiveDateTime,
    orderno: i32,
    action: Action,
    price: Price,
    volume: i64,
}

impl EquityOrderLog {
    pub fn new(s: &str) -> Option<Self> {
        let mut iter = s.split(",");
        let timestamp_fmt = "%Y%m%d%H%M%S%f";

        let _sequence_number = iter.next()?.parse::<i64>().ok()?;
        let sec_code = iter.next()?.to_string();
        let buy_sell = Side::from_str(iter.next()?).ok()?;
        let time = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
        let order_no = iter.next()?.parse::<i64>().ok()?;
        let action_byte = iter.next()?;
        let price = {
            let n = iter.next()?;
            let decimal = iter.next()?.parse::<Decimal>().ok()?;
            if n == "0" {
                Price::Market
            } else {
                Price::Limit(decimal)
            }
        };
        let volume = iter.next()?.parse::<i64>().ok()?;
        let action = match iter.next()? {
            "" if action_byte == "0" => Action::Cancel,
            "" if action_byte == "1" => Action::Add,
            trade_id => {
                let price = iter.next()?.parse::<i64>().ok()?;
                let id = trade_id.parse::<i64>().ok()?;
                Action::Trade(TradeLog { price, id })
            }
        };
        Some(Self {
            no,
            seccode,
            buysell,
            time,
            orderno,
            action,
            price,
            volume,
        })
    }
}
