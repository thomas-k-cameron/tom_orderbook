use std::{path::PathBuf, collections::HashMap};

use dbn::{decode::{DecodeDbn, self}, record::{MboMsg, str_to_c_chars, c_chars_to_str, TradeMsg, HasRType}, enums::{flags, Action}, error::ConversionError};
use tom_orderbook::{MakerOrder, OrderBook};

struct Config {
    data_file: PathBuf
}

fn main() {
    let mut file = dbn::decode::dbn::Decoder::from_zstd_file("../proprietary-data/databento/glbx-mdp3-20230221.mbo.dbn.zst").unwrap();
    let mut hashmap = HashMap::new();
    let mut orderbook = HashMap::new();
    loop {
        match file.decode_record::<MboMsg>() {
            Ok(Some(r)) => {
                let action = Action::try_from(r.action as u8)
                .map_err(|_| ConversionError::TypeConversion("Invalid action"));
                *hashmap.entry(format!("{:?}", action.unwrap())).or_insert(1) += 1;
            },
            Ok(None) => {
                println!("{hashmap:#?}");
                break
            },
            Err(e) => {
                println!("{e:#?}");
                println!("{hashmap:#?}");
                break
            }
        };

        //let action = Action::try_from(i.action as u8).unwrap();
        //if action == Action::Fill {
            //println!("{i:#?}")
        //}

        /*
        let id = i.order_id as u64;
        let mut_book = hashmap.entry(i.hd.instrument_id).or_insert_with(|| {
            OrderBook::new(i.hd.instrument_id as u64)
        });

        let action = Action::try_from(i.action as u8).unwrap();
        
        match action {
            Action::Add => mut_book.add(order),
            Action::Cancel => mut_book.remove(&id),
            Action::Fill => mut_book.replace(add, ),
        }
        mut_book.add(order)
        println!("{action:#?}");
    */
    }
    
}
