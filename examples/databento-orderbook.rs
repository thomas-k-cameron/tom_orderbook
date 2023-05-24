use std::{path::PathBuf, collections::HashMap};

use dbn::{decode::{DecodeDbn, self}, record::{MboMsg, str_to_c_chars, c_chars_to_str}, enums::{flags, Action}};
use tom_orderbook::{MakerOrder, OrderBook};

struct Config {
    data_file: PathBuf
}

fn main() {
    let mut file = dbn::decode::dbn::Decoder::from_zstd_file("proprietary-data/databento/glbx-mdp3-20230221.mbo.dbn.zst").unwrap();
    let mut hashmap = HashMap::new();
    loop {
        let item = file.decode_record();
        
        let i: &MboMsg = match item {
            Ok(Some(i)) => i,
            _ => {
                println!("{item:#?}");
                continue;
            }
        };
        let mut_book = hashmap.entry(i.hd.instrument_id).or_insert_with(|| {
            OrderBook::new(i.hd.instrument_id as u64)
        });

        let action = Action::try_from(i.action as u8).unwrap();
        let order = ;
        match action {
            Action::Add => mut_book.add(order),
            Action::Cancel => ,
            Action::Fill => ,
        }
        mut_book.add(order)
        println!("{action:#?}");
    }
} 