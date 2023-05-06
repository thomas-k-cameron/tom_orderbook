use crate::DerivativeOrderLog;

#[test]
fn opt_log_deal() {
    let s = include_str!("../test-data/head.test.txt");
    for i in s.split("\n") {
        let log = DerivativeOrderLog::new(s).unwrap();
        println!("{log:#?}");
    }
}