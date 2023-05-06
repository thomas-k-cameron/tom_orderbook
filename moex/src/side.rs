use std::str::FromStr;

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}
impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "B" => Side::Buy,
            "S" => Side::Sell,
            _ => return Err(()),
        })
    }
}
