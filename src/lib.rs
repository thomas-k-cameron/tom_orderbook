use market_datatypes::{OrderId, OrderPrice, Price, Side};
use std::{
    collections::{HashMap, VecDeque},
    fs::ReadDir,
    ops::Deref,
};

/// Order that bridges between the origianl order and the order within the order book
pub struct MakerOrder {
    pub id: u64,
    pub price: OrderPrice<i64>,
    pub qty: i64,
    pub side: Side,
}

pub struct PriceQty {
    price: OrderPrice<i64>,
    qty: i64,
}

#[derive(Default)]
pub struct PriceLevel {
    order_stack: HashMap<u64, MakerOrder>,
    insertion_order: Vec<u64>,
    pub(crate) price: OrderPrice<i64>,
    qty: i64,
}
impl PriceLevel {
    pub fn price_min_if_market(&self) -> i64 {
        if let OrderPrice::Limit(i) = self.price {
            i
        } else {
            i64::MIN
        }
    }
    pub fn is_market_order_stack(&self) -> bool {
        matches!(self.price, OrderPrice::Market)
    }
    pub fn is_limit_order_stack(&self) -> bool {
        matches!(self.price, OrderPrice::Limit(_))
    }
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
    pub fn remove(&mut self, id: &u64) -> Option<MakerOrder> {
        match self.order_stack.remove(id) {
            Some(i) => {
                self.qty -= i.qty;
                Some(i)
            }
            None => None,
        }
    }
    // TODO bad name. change it
    pub fn shrink_queue(&mut self) {
        let new = self
            .insertion_order
            .drain(..)
            .filter(|i| self.order_stack.contains_key(i))
            .collect();
        self.insertion_order.clone_from(&new);
    }
    pub fn price_qty(&self) -> PriceQty {
        PriceQty {
            price: self.price.clone(),
            qty: self.qty.clone(),
        }
    }
    /// iterate maker orders
    pub fn iter_orders(&self) -> impl Iterator<Item = (&u64, &MakerOrder)> {
        self.insertion_order
            .iter()
            .filter_map(|id| match self.order_stack.get(id) {
                Some(ord) => Some((id, ord)),
                None => None,
            })
    }
}

#[derive(Default)]
pub struct OrderBook {
    order_book_id: u64,
    /// stack for ask orders
    ask_orders: VecDeque<PriceLevel>,
    /// stack for bid orders
    bid_orders: VecDeque<PriceLevel>,
    /// allows you to look up the location of the order by it's id
    order_lookup: HashMap<u64, (OrderPrice<i64>, Side)>,
    ask_market_orders: PriceLevel,
    bid_market_orders: PriceLevel,
}

impl OrderBook {
    pub fn order_book_id(&self) -> u64 {
        self.order_book_id
    }

    pub fn new(order_book_id: u64) -> Self {
        let mut data: Self = Default::default();
        data.order_book_id = order_book_id;
        data
    }

    fn mut_price_level(
        &mut self,
        price: &OrderPrice<i64>,
        side: &Side,
    ) -> Result<&mut PriceLevel, usize> {
        match price {
            OrderPrice::Limit(price) => {
                let stack = match side {
                    Side::Buy => &mut self.ask_orders,
                    Side::Sell => &mut self.bid_orders,
                };
                let item = stack.binary_search_by(|i| i.price.price_min_if_market().cmp(&price));
                match item {
                    Ok(idx) => Ok(&mut stack[idx]),
                    Err(idx) => Err(idx),
                }
            }
            OrderPrice::Market => match side {
                Side::Buy => Ok(&mut self.ask_market_orders),
                Side::Sell => Ok(&mut self.bid_market_orders),
            },
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

    pub fn remove(&mut self, id: &UniqueOrderId) -> Result<(), ()> {
        let (price, side) = self.order_lookup.remove(&id.0).unwrap();
        match self.mut_price_level(&price, &side) {
            Ok(level) => level.remove(&id.0),
            Err(_idx) => return Err(()),
        };
        Ok(())
    }

    pub fn change_qty(&mut self, target_id: UniqueOrderId, change_qty: i64) -> Result<(), ()> {
        if let Some((price, side)) =  self.order_lookup.get(&target_id) {
            match self.mut_price_level(price, side) {
                Ok(i) => {
                    if let Some(mut ord) = i.remove(&target_id) {
                        ord.qty = change_qty;
                        i.add(ord);
                        return Ok(())
                    }
                }
                _ => ()
            }
        }
        Err(())
    }

    pub fn replace(&mut self, add: MakerOrder, remove: UniqueOrderId) -> Result<(), ()> {
        self.remove(&remove)?;
        self.add(add);
        Ok(())
    }
}

pub struct UniqueOrderId(u64);

impl UniqueOrderId {
    pub fn new(i: u64) -> Self {
        Self(i)
    }
}

impl From<u64> for UniqueOrderId {
    fn from(value: u64) -> Self {
        UniqueOrderId(value)
    }
}

impl Deref for UniqueOrderId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum OrderBookUpdate {
    ChangeQty {
        target_order: UniqueOrderId,
        new_qty: i64,
    },
    ChangePrice {
        new_price: i64,
        target_order: UniqueOrderId,
    },
    Delete {
        target_order: UniqueOrderId,
    },
    Add(MakerOrder),
}