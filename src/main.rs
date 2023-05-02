type Qty = i64;
type Price = i64;
type OrderId = u64;
use std::collections::{BTreeMap, HashMap, VecDeque};

fn main() {
    println!("Hello, world!");
}

struct MakerOrder {
    id: u64,
    price: i64,
    qty: i64,
}
struct JPXItch {
    ask: BTreeMap<i64, Vec<MakerOrder>>,
    bid: BTreeMap<i64, Vec<MakerOrder>>,
}

trait Order: OrderIdentifier {
    const DATA_VARIATION: DataVariation;
    fn price(&self) -> i64;
    fn qty(&self) -> i64;
    fn side(&self) -> Side;
}

trait OrderIdentifier {
    fn order_id(&self) -> u64;
}

#[derive(Clone, Copy)]
enum Side {
    Buy,
    Sell,
}

enum DataVariation {
    Level1,
    Level2,
}

struct OrderBook<O: Order> {
    ask_orders: VecDeque<PriceLevel<O>>,
    bid_orders: VecDeque<PriceLevel<O>>,
    order_lookup: HashMap<OrderId, (Price, Side)>,
}


pub struct PriceQty {
    pub price: i64,
    pub qty: i64,
}

impl<O: Order> From<&O> for PriceQty {
    fn from(value: &O) -> Self {
        Self {
            price: value.price(),
            qty: value.qty(),
        }
    }
}

impl<O: Order> OrderBook<O> {
    fn mut_price_level(&mut self, price: &i64, side: &Side) -> Result<&mut PriceLevel<O>, usize> {
        let stack = match side {
            Side::Buy => &mut self.ask_orders,
            Side::Sell => &mut self.bid_orders,
        };
        let item = stack.binary_search_by(|i| i.price.cmp(&price));
        match item {
            Ok(idx) => Ok(&mut stack[idx]),
            Err(idx) => Err(idx),
        }
    }

    pub fn add(&mut self, order: O) -> PriceQty {
        let side = order.side();
        match self.mut_price_level(&order.price(), &side) {
            Ok(level) => {
                level.add(order)
            }
            Err(idx) => {
                let ret = (&order).into();
                match side {
                    Side::Buy => self
                        .bid_orders
                        .insert(idx, PriceLevel::new_with_order(order)),
                    Side::Sell => self
                        .ask_orders
                        .insert(idx, PriceLevel::new_with_order(order)),
                };
                ret
            }
        }
    }

    pub fn remove(&mut self, order: impl Order) -> Result<(), ()> {
        let (price, side) = self.order_lookup.get(&order.order_id()).unwrap();
        let (price, side) = (*price, *side);
        match self.mut_price_level(&price, &side) {
            Ok(level) => level.remove(order.order_id()),
            Err(_idx) => return Err(()),
        };
        Ok(())
    }

    pub fn replace(&mut self, add: O, remove: impl Order) -> Result<(), ()> {
        self.remove(remove)?;
        self.add(add);
        Ok(())
    }
}

struct PriceLevel<O: Order> {
    order_stack: HashMap<OrderId, O>,
    price: i64,
    qty: i64,
}

impl<O: Order> PriceLevel<O> {
    fn new_with_order(order: O) -> Self {
        Self {
            price: order.price(),
            qty: order.qty(),
            order_stack: {
                let mut map = HashMap::with_capacity(1);
                map.insert(order.order_id(), order);
                map
            },
        }
    }
    fn add(&mut self, o: O) -> PriceQty {
        self.qty += o.qty();
        let n = self.order_stack.insert(o.order_id(), o);
        assert!(n.is_none());
        PriceQty {
            price: self.price,
            qty: self.qty,
        }
    }
    fn remove(&mut self, id: OrderId) -> Option<O> {
        match self.order_stack.remove(&id) {
            Some(i) => {
                self.qty -= i.qty();
                Some(i)
            }
            None => None,
        }
    }
    fn replace(&mut self, o: O) -> Option<O> {
        self.qty += o.qty();
        if let Some(removed) = self.order_stack.insert(o.order_id(), o) {
            self.qty -= removed.qty();
            Some(removed)
        } else {
            None
        }
    }
    // use it for executions
    fn reduce_qty(&mut self, order_id: OrderId) -> Result<(), ()> {
        if let Some(ord) = self.order_stack.get_mut(&order_id) {
            self.qty -= ord.qty();
            Ok(())
        } else {
            Err(())
        }

    }
}
