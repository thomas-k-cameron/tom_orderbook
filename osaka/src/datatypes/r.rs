// automatically generated

use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::util::extract_datetime;
use crate::{
    tag_guard,
    FinancialProduct,
    PutOrCall,
};

///
/// 6.3.2 銘柄情報基本タグ （タグ ID ： R ）  
///
/// （１） タグ内容  
///
/// 当日営業日に取引可能な銘柄の詳細情報を提供する（取引停止中の銘柄も含む。）。  
///
/// （２） タグ出力タイミング  
///
/// オンライン開始後、一定時間経過後に提供する。  
///
/// テーラーメイドコンビネーション(TMC)が組成されたときに提供する。  
///
/// J-GATE 内で銘柄情報の更新が発生したときに提供する。(R タグで出力していない情報が更新された時にも提供する場合がある。)  
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
pub struct ProductInfo {
    pub timestamp: NaiveDateTime,
    pub expiration_date: i64,
    pub financial_product: FinancialProduct,
    pub long_name: String,
    pub number_of_decimal_in_price: i64,
    pub number_of_decimals_in_strike_price: i64,
    pub number_of_legs: i64,
    pub order_book_id: i64,
    pub put_or_call: PutOrCall,
    pub strike_price: i64,
    pub symbol: String,
    pub underlying_order_book_id: i64,
}

impl_message! {
    name: ProductInfo 'R';
    pub timestamp: NaiveDateTime,
    pub expiration_date: i64,
    pub financial_product: FinancialProduct,
    pub long_name: String,
    pub number_of_decimal_in_price: i64,
    pub number_of_decimals_in_strike_price: i64,
    pub number_of_legs: i64,
    pub order_book_id: i64,
    pub put_or_call: PutOrCall,
    pub strike_price: i64,
    pub symbol: String,
    pub underlying_order_book_id: i64,
}

impl TryFrom<&str> for ProductInfo {
    type Error = ();

    //parse_row!(@ parse_r, ["_","timestamp","order_book_id","symbol","long_name","_reserved","financial_product","_trading_currency","number_of_decimal_in_price","_nominal_value",
    // "_odd_lot_size","_round_lot_size","_block_lot_size","_nominal_value","number_of_legs","underlying_order_book_id","strike_price","expiration_date","number_of_decimals_in_strike_price","put_or_call"]);
    //R,2021-03-30T21:14:49.816929242(1617138889816929242),590334,FUT_NK225M_2109,166090019,166090019,3,JPY,4,0,0,1,0,0,0,510,0,20210910,0,0

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        tag_guard!('R', s);
        let mut iter = s.split(",").skip(1);
        let timestamp = extract_datetime(iter.next().ok_or(())?).ok_or(())?;

        let order_book_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let symbol = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        let long_name = iter.next().ok_or(())?.to_string();
        let _reserved = iter.next();
        let financial_product = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let _trading_currency = iter.next();
        let number_of_decimal_in_price = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        // iter.next
        let _nominal_value = iter.next();
        let _odd_lot_size = iter.next();
        let _round_lot_size = iter.next();
        let _block_lot_size = iter.next();
        let _nominal_value = iter.next();

        let number_of_legs = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let underlying_order_book_id = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let strike_price = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let expiration_date = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;
        let number_of_decimals_in_strike_price =
            FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        let put_or_call = FromStr::from_str(iter.next().ok_or(())?).ok().ok_or(())?;

        Ok(Self {
            timestamp,
            expiration_date,
            financial_product,
            long_name,
            number_of_decimal_in_price,
            number_of_decimals_in_strike_price,
            number_of_legs,
            order_book_id,
            put_or_call,
            strike_price,
            symbol,
            underlying_order_book_id,
        })
    }
}
