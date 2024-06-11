use crate::core::order;
use bplustree::BPlusTree;
use rust_decimal::Decimal;
use std::{collections::HashMap, ops::Deref, sync};
pub struct Matcher {
    conf: Config,
    buy_orders: BPlusTree<order::Order, *mut order::Order>,
    sell_orders: BPlusTree<order::Order, *mut order::Order>,
    //store order info by id
    idbook: HashMap<String, *mut order::Order>,
    // id_orders: BPlusTree<String, order::Order>,
}
#[derive(Clone)]
pub struct Config {
    _name: &'static str,
}
impl Copy for Config {}
// fn _1() {
//     let lock = sync::Arc::new(sync::Mutex::new(0));
// }

enum OrderType {
    Buy = 1,
    Sell,
}
impl Matcher {
    pub fn new(conf: &Config) -> Self {
        return Self {
            conf: conf.clone(),
            buy_orders: BPlusTree::new(),
            sell_orders: BPlusTree::new(),
            idbook: HashMap::new(),
        };
    }
    /// add a order,mark it 1 as buy,2 as sell
    pub fn add_order(&mut self, _type: OrderType, order: &super::order::Order) {
        let orderonheap: Box<order::Order> = Box::new(order.clone());
        let rptr = Box::leak(orderonheap) as *mut super::order::Order;
        match _type {
            OrderType::Buy => {
                self.buy_orders.insert(order.clone(), rptr);
            }
            OrderType::Sell => {
                self.sell_orders.insert(order.clone(), rptr);
            }
        }
        self.idbook.insert(order.get_id(), rptr);
    }
}

///matcher
pub mod factory {
    use std::ops::{Div, Sub};

    use bplustree::iter::RawSharedIter;
    use rust_decimal::{
        prelude::{FromPrimitive, Zero},
        Decimal,
    };
    struct Matcher {}

    impl Matcher {
        pub fn new() -> Self {
            panic!("not  implement");
        }
        pub fn match_orders(
            &mut self,
            mut buy_from: RawSharedIter<'_, Decimal, super::order::Order, 128, 256>,
            mut sell_from: RawSharedIter<'_, Decimal, super::order::Order, 128, 256>,
        ) {
            buy_from.seek_to_last();
            sell_from.seek_to_last();
            let mut buyorder = buy_from.prev();
            let mut sellorder = sell_from.prev();
            while buyorder.is_some() && sellorder.is_some() {
                let mut ordinfo = buyorder.unwrap();
                let mut selinfo = sellorder.unwrap();
                let mut dealorder_price = (ordinfo.0 + selinfo.0).div(Decimal::from_i8(2).unwrap());
                let mut dealamount = ordinfo.1._amount;
                if dealamount >= selinfo.1._amount {
                    dealamount = dealamount.sub(selinfo.1._amount);
                    //todo: sub data and store to map
                    // selinfo.1._amount = Decimal::zero();
                }
            }
        }
    }
}
