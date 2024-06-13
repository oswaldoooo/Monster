use std::sync::mpsc::{channel, Receiver, Sender};

use bplustree::BPlusTree;

pub mod matcher;
pub mod node;
pub mod order;
#[path = "../runtime/mod.rs"]
pub mod runtime;

//total core,use to global
pub struct Core {
    sub_buy_book: bplustree::BPlusTree<String, Sender<u8>>,
    sub_sell_book: bplustree::BPlusTree<String, Sender<u8>>,
    buy_orders: order::factory::Order,
    sell_orders: order::factory::Order,
}
impl Core {
    pub fn new() -> Self {
        Self {
            sub_buy_book: BPlusTree::new(),
            sub_sell_book: BPlusTree::new(),
            buy_orders: order::factory::Order::new(),
            sell_orders: order::factory::Order::new(),
        }
    }
    pub fn add_buy_one(&mut self, orderId: String) -> Option<Receiver<u8>> {
        if !self.buy_orders.contains(orderId.clone()) {
            return None;
        }
        let (sx, rx) = channel();
        self.sub_buy_book.insert(orderId.clone(), sx);
        Some(rx)
    }

    //match order at backend
    pub async fn match_order(&mut self) {}
}
