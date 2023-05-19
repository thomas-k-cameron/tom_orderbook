use crate::*;

pub struct PriceLevel<O: Order> {
    order_stack: HashMap<OrderId, O>,
    pub(crate) price: i64,
    qty: i64,
}

impl<O: Order> PriceLevel<O> {
    pub fn new_with_order(order: O) -> Self {
        Self {
            price: order.price(),
            qty: order.qty(),
            order_stack: {
                let mut map = HashMap::with_capacity(1000);
                map.insert(order.unique_order_id(), order);
                map
            },
        }
    }
    pub fn add(&mut self, o: O) {
        self.qty += o.qty();
        self.order_stack.insert(o.unique_order_id(), o);
    }
    pub fn remove(&mut self, id: OrderId) -> Option<O> {
        match self.order_stack.remove(&id) {
            Some(i) => {
                self.qty -= i.qty();
                Some(i)
            }
            None => None,
        }
    }
    pub fn price_qty(&self) -> PriceQty {
        PriceQty {
            price: self.price,
            qty: self.qty,
        }
    }
}
