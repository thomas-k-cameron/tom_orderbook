use market_datatypes::Side;


pub trait Order: UniqueOrderId {
    fn price(&self) -> i64;
    fn qty(&self) -> i64;
    fn side(&self) -> Side;
}

pub trait UniqueOrderId {
    fn unique_order_id(&self) -> u64;
}