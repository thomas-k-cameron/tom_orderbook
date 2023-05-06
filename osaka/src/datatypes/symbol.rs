use std::convert::Infallible;
use std::str::FromStr;

use chrono::NaiveDate;

use crate::{
    FinancialProduct,
    PutOrCall,
};

//PUT_8473_210909_3600
//CAL_TOPIX_211210_2050
//NK225M_2105

pub struct Symbol {
    pub financial_product: FinancialProduct,
    pub put_or_call: PutOrCall,
    pub product_name: String,
    pub strike_price: i64,
    pub number_of_decimals_in_strike_price: i64,
    pub expiration: NaiveDate,
    pub combo_with: Box<Option<Symbol>>,
}

impl FromStr for Symbol {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // futures
        let mut item = if s.contains("/") {
            (&s[4..]).split("/").map(parse_item).collect()
        } else {
            vec![parse_item(s)]
        };

        if item.len() == 2 {
            item[0].combo_with = Box::new(item.pop());
        }
        Ok(item.pop().ok_or(()).unwrap())
    }
}

fn parse_item(s: &str) -> Symbol {
    let n = s.chars().filter(|c| *c == '_').count();
    let mut iter = s.split("_");
    match n {
        1 => {
            Symbol {
                financial_product: FinancialProduct::Future,
                product_name: iter.next().unwrap().to_string(),
                expiration: {
                    let s = iter.next().unwrap();
                    NaiveDate::from_ymd(
                        2000i32 + (&s[0..2]).parse::<i32>().unwrap(),
                        s[3..].parse().unwrap(),
                        1,
                    )
                },
                put_or_call: PutOrCall::Combo,
                strike_price: 0,
                number_of_decimals_in_strike_price: 0,
                combo_with: Box::new(None),
            }
        }
        4 => {
            Symbol {
                financial_product: FinancialProduct::Option,
                put_or_call: iter.next().unwrap().to_string().parse().unwrap(),
                product_name: iter.next().unwrap().to_string(),
                expiration: {
                    let s = iter.next().unwrap();
                    NaiveDate::from_ymd(
                        2000i32 + (&s[0..2]).parse::<i32>().unwrap(),
                        s[3..].parse().unwrap(),
                        1,
                    )
                },
                strike_price: 0,
                number_of_decimals_in_strike_price: 0,
                combo_with: Box::new(None),
            }
        }
        _ => unreachable!(),
    }
}
