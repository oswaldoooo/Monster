use rust_decimal::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum OrderStatus {
    Runing,
    Ok,
    InMatching, //prepare match
}
enum OrderType {
    Buy = 1,
    Sell,
}
#[derive(Ord, Eq, Debug)]
pub struct Order {
    ///order id
    _id: String,
    ///order price
    _price: Decimal,
    ///order amount
    pub _amount: Decimal,
    _status: OrderStatus,
}
impl Clone for Order {
    fn clone(&self) -> Self {
        Self {
            _id: self._id.clone(),
            _price: self._price.clone(),
            _amount: self._amount.clone(),
            _status: self._status.clone(),
        }
    }
}

impl Order {
    pub fn new(_id: String, _price: Decimal, _amount: Decimal) -> Self {
        return Self {
            _id: _id,
            _price: _price,
            _amount: _amount,
            _status: OrderStatus::Runing,
        };
    }
    pub fn get_price(&self) -> Decimal {
        return self._price.clone();
    }
    pub fn get_id(&self) -> String {
        return self._id.clone();
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        // println!("compare {} == {}", self._id, other._id);
        return self._id == other._id;
    }
}
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // println!("call cmp");
        if self.lt(other) {
            return Some(std::cmp::Ordering::Less);
        } else if self.gt(other) {
            return Some(std::cmp::Ordering::Greater);
        }
        return Some(std::cmp::Ordering::Equal);
    }
    fn lt(&self, other: &Self) -> bool {
        // println!("call lt");
        if self._price < other._price {
            return true;
        } else if self._price == other._price && self._amount < other._amount {
            return true;
        }
        return false;
    }
    fn gt(&self, other: &Self) -> bool {
        return !self.le(other);
    }
    fn le(&self, other: &Self) -> bool {
        return self.lt(other) && self.eq(other);
    }
    fn ge(&self, other: &Self) -> bool {
        return !self.lt(other);
    }
}
// factory mod
pub mod factory {
    use bplustree::BPlusTree;

    use crate::core::node::{self, Node};
    use std::collections::HashMap;
    pub struct Order {
        buy_orders: BPlusTree<super::Order, *mut super::Order>,
        sell_orders: BPlusTree<super::Order, *mut super::Order>,
        //store order info by id
        idbook: HashMap<String, *mut super::Order>,
        // id_orders: BPlusTree<String, super::Order>,
        _match_len: u64, //single max match length
    }
    impl Order {
        pub fn new() -> Self {
            panic!("not implement");
        }
        // 返回订单id
        pub fn add(&mut self, _order: super::Order) -> String {
            return String::default();
        }
        pub fn cancle(&mut self, _orderid: String) {}
        /// add a order,mark it 1 as buy,2 as sell
        pub fn add_order(&mut self, _type: super::OrderType, _order: &super::Order) {
            let orderonheap: Box<super::Order> = Box::new(_order.clone());
            let rptr = Box::leak(orderonheap) as *mut super::Order;
            match _type {
                super::OrderType::Buy => {
                    self.buy_orders.insert(_order.clone(), rptr);
                }
                super::OrderType::Sell => {
                    self.sell_orders.insert(_order.clone(), rptr);
                }
            }
            self.idbook.insert(_order.get_id(), rptr);
        }
        pub fn cancle_order(&mut self, orderId: String) -> TaskStatus {
            let ord = self.idbook.remove(&orderId);
            if ord.is_none() {
                return TaskStatus::Invalid;
            }
            let rdata = ord.unwrap();
            unsafe {
                match (*rdata)._status {
                    super::OrderStatus::Runing => {
                        self.buy_orders.remove(&*rdata);
                        self.sell_orders.remove(&*rdata);
                    }
                    super::OrderStatus::Ok => {
                        return TaskStatus::Failed;
                    }
                    super::OrderStatus::InMatching => {
                        return TaskStatus::Failed;
                    }
                }
            }
            return TaskStatus::Succ;
        }
    }

    enum TaskStatus {
        Succ,
        Failed,
        Invalid,
        Lllegal,
    }
}
