use rust_decimal::prelude::*;
use std::collections::LinkedList;
pub struct Order {
    ///order id
    _id: String,
    ///order price
    _price: Decimal,
    ///order amount
    pub _amount: Decimal,
}
impl Clone for Order {
    fn clone(&self) -> Self {
        Self {
            _id: self._id.clone(),
            _price: self._price.clone(),
            _amount: self._amount.clone(),
        }
    }
}
impl Order {
    pub fn new(_id: String, _price: Decimal, _amount: Decimal) -> Self {
        return Self {
            _id: _id,
            _price: _price,
            _amount: _amount,
        };
    }
    pub fn get_price(&self) -> Decimal {
        return self._price.clone();
    }
}
// factory mod
pub mod factory {
    use crate::core::node::{self, Node};
    pub struct Order {
        _head: Option<Node<super::Order>>, //head node
        _tail: Option<Node<super::Order>>, //tail node
        _match_len: u64,                   //single max match length
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
    }
}
