use crate::*;

pub struct PriceLevel<O: Order> {
    order_stack: HashMap<OrderId, O>,
    insertion_order: Vec<OrderId>,
    pub(crate) price: i64,
    qty: i64,
}

impl<O: Order> PriceLevel<O> {
    pub fn new_with_order(order: O) -> Self {
        Self {
            price: order.price(),
            qty: order.qty(),
            insertion_order: {
                vec![order.unique_order_id()]
            },
            order_stack: {
                let mut map = HashMap::with_capacity(1000);
                map.insert(order.unique_order_id(), order);
                map
            },
        }
    }
    pub fn add(&mut self, o: O) {
        self.qty += o.qty();
        self.insertion_order.push(o.unique_order_id());
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
    /// iterate maker orders
    pub fn iter_orders(&self) -> impl Iterator<Item = (&OrderId, &O)> {
        self.insertion_order.iter().filter_map(|id| {
            match self.order_stack.get(id) {
                Some(ord) => Some((id, ord)),
                None => None
            }
        })
    }
}
