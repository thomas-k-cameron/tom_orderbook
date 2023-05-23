use crate::*;


pub struct OrderBook<O: Order> {
    order_book_id: u64,
    /// stack for ask orders
    ask_orders: VecDeque<PriceLevel<O>>,
    /// stack for bid orders
    bid_orders: VecDeque<PriceLevel<O>>,
    /// allows you to look up the location of the order by it's id
    order_lookup: HashMap<OrderId, (Price, Side)>,
}


impl<O: Order> OrderBook<O> {
    pub fn order_book_id(&self) -> u64 {
        self.order_book_id.clone()
    }

    pub fn new(order_book_id: u64) -> Self {
        Self {
            order_book_id,
            ask_orders: Default::default(),
            bid_orders: Default::default(),
            order_lookup: Default::default(),
        }
    }

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

    /// iterates the price level with PriceQty
    pub fn iter_orders<'a>(&'a self, side: &Side) -> impl Iterator<Item = PriceQty> + 'a {
        let iter = match side {
            Side::Buy => self.bid_orders.iter(),
            Side::Sell => self.ask_orders.iter(),
        };
        iter.map(|i| i.price_qty())
    }

    /// iterates the price level
    pub fn iter_price_level<'a>(&'a self, side: &Side) -> impl Iterator<Item = &PriceLevel<O>> + 'a {
        let iter = match side {
            Side::Buy => self.bid_orders.iter(),
            Side::Sell => self.ask_orders.iter(),
        };
        iter
    }

    pub fn add(&mut self, order: O) {
        let side = order.side();
        match self.mut_price_level(&order.price(), &side) {
            Ok(level) => level.add(order),
            Err(idx) => {
                let val = PriceLevel::new_with_order(order);
                match side {
                    Side::Buy => self
                        .bid_orders
                        .insert(idx, val),
                    Side::Sell => self
                        .ask_orders
                        .insert(idx, val),
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

    pub fn replace(&mut self, add: O, remove: impl UniqueOrderId) -> Result<(), ()> {
        self.remove(remove)?;
        self.add(add);
        Ok(())
    }
}

impl<O: Order> OrderBook<O> {
    pub fn update(&mut self, msg: OrderBookUpdate) { 
        
    }
}

pub enum OrderBookUpdate<O: Order, U: UniqueOrderId> {
    Add(O),
    Delete(U),
    Update(O, U),
    Execution,
}