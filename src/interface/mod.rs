use std::collections::{HashMap, VecDeque};

use market_datatypes::{OrderId, Side};

pub trait Order: UniqueOrderId {
    fn price(&self) -> i64;
    fn qty(&self) -> i64;
    fn side(&self) -> Side;
}
pub trait UniqueOrderId {
    fn unique_order_id(&self) -> u64;
}

pub trait Execution {
    fn maker_id(&self) -> u64;
    fn size(&self) -> i64;
}

/// Order that bridges between the origianl order and the order within the order book
pub struct MakerOrder {
    pub id: u64,
    pub price: i64,
    pub qty: i64,
    pub side: Side,
}

pub struct PriceQty {
    price: i64,
    qty: i64,
}

pub struct PriceLevel {
    order_stack: HashMap<OrderId, MakerOrder>,
    insertion_order: Vec<OrderId>,
    pub(crate) price: i64,
    qty: i64,
}

impl PriceLevel {
    pub fn new_with_order(order: MakerOrder) -> Self {
        Self {
            price: order.price,
            qty: order.qty,
            insertion_order: { vec![order.id] },
            order_stack: {
                let mut map = HashMap::with_capacity(1000);
                map.insert(order.id, order);
                map
            },
        }
    }
    pub fn add(&mut self, o: MakerOrder) {
        self.qty += o.qty;
        self.insertion_order.push(o.id);
        self.order_stack.insert(o.id, o);
    }
    pub fn remove(&mut self, id: OrderId) -> Option<MakerOrder> {
        match self.order_stack.remove(&id) {
            Some(i) => {
                self.qty -= i.qty;
                Some(i)
            }
            None => None,
        }
    }
    // TODO bad name change it
    pub fn shrink_queue(&self) {
        self.insertion_order.clone_from(
            self.insertion_order
                .drain(..)
                .filter(|i| self.order_stack.contains_key(i))
                .collect(),
        );
    }
    pub fn price_qty(&self) -> PriceQty {
        PriceQty {
            price: self.price.clone(),
            qty: self.qty.clone(),
        }
    }
    /// iterate maker orders
    pub fn iter_orders(&self) -> impl Iterator<Item = (&OrderId, &MakerOrder)> {
        self.insertion_order
            .iter()
            .filter_map(|id| match self.order_stack.get(id) {
                Some(ord) => Some((id, ord)),
                None => None,
            })
    }
}

pub struct OrderBook {
    order_book_id: u64,
    /// stack for ask orders
    ask_orders: VecDeque<PriceLevel>,
    /// stack for bid orders
    bid_orders: VecDeque<PriceLevel>,
    /// allows you to look up the location of the order by it's id
    order_lookup: HashMap<OrderId, (i64, Side)>,
}

impl OrderBook {
    pub fn order_book_id(&self) -> u64 {
        self.order_book_id
    }

    pub fn new(order_book_id: u64) -> Self {
        Self {
            order_book_id,
            ask_orders: Default::default(),
            bid_orders: Default::default(),
            order_lookup: Default::default(),
        }
    }

    fn mut_price_level(&mut self, price: &i64, side: &Side) -> Result<&mut PriceLevel, usize> {
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

    /// iterates the price level with PriceQty
    pub fn iter_orders<'a>(&'a self, side: &Side) -> impl Iterator<Item = PriceQty> + 'a {
        let iter = match side {
            Side::Buy => self.bid_orders.iter(),
            Side::Sell => self.ask_orders.iter(),
        };
        iter.map(|i| i.price_qty())
    }

    /// iterates the price level
    pub fn iter_price_level<'a>(&'a self, side: &Side) -> impl Iterator<Item = &PriceLevel> + 'a {
        let iter = match side {
            Side::Buy => self.bid_orders.iter(),
            Side::Sell => self.ask_orders.iter(),
        };
        iter
    }

    pub fn add(&mut self, order: MakerOrder) {
        let side = order.side;
        match self.mut_price_level(&order.price, &side) {
            Ok(level) => level.add(order),
            Err(idx) => {
                let val = PriceLevel::new_with_order(order);
                match side {
                    Side::Buy => self.bid_orders.insert(idx, val),
                    Side::Sell => self.ask_orders.insert(idx, val),
                };
            }
        }
    }

    pub fn remove(&mut self, id: impl UniqueOrderId) -> Result<(), ()> {
        let (price, side) = self.order_lookup.remove(&id.unique_order_id()).unwrap();
        match self.mut_price_level(&price, &side) {
            Ok(level) => level.remove(id.unique_order_id()),
            Err(_idx) => return Err(()),
        };
        Ok(())
    }

    pub fn replace(&mut self, add: MakerOrder, remove: impl UniqueOrderId) -> Result<(), ()> {
        self.remove(remove)?;
        self.add(add);
        Ok(())
    }
}
