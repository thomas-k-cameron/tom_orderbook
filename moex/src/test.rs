use std::collections::HashMap;

use tom_orderbook::OrderBook;

use crate::{DerivativeOrderLog, OrderBookId, TradeLog};

#[test]
fn opt_log_deal() {
    let s = include_str!("../test-data/head.test.txt");
    for i in s.split("\n") {
        let log = DerivativeOrderLog::new(s).unwrap();
        println!("{log:#?}");
    }
}

#[test]
fn order_book() {
    let file = include_str!("../test-data/5000.txt");
    
    let mut hashmap = HashMap::new();
    
    for i in file.split("\n") {
        let log  =DerivativeOrderLog::new(i).unwrap();
        let mut book = OrderBook::new(hashmap.len() as u64);
        hashmap.insert(log.name, hashmap.len());
        match log.action {
            crate::Action::Add => book.add(log),
            crate::Action::Cancel => (),
            crate::Action::Trade(log) => {
                (   )
            }
        }
    }
    
}