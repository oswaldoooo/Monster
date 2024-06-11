use std::str::FromStr;

use crate::core::order;
use bplustree::BPlusTree;
use rust_decimal::Decimal;
#[test]
fn test_orders() {
    let mut iin: BPlusTree<order::Order, order::Order> = BPlusTree::new();
    init_orders(&iin);
    checkorders(&iin);
    let ordinfo = order::Order::new("1001".to_string(), Decimal::default(), Decimal::default());
    let ordinfo2 = order::Order::new(
        "1001".to_string(),
        Decimal::default(),
        Decimal::from_str("123").unwrap(),
    );
    let dt = iin.lookup(&ordinfo, getorder);
    if dt.is_some() {
        println!("find {:?}", dt.unwrap());
    } else {
        println!("not found");
    }
    // println!("is equal {}", ordinfo == ordinfo2);
}
fn checkorders(input: &BPlusTree<order::Order, order::Order>) {
    let mut iter = input.raw_iter();
    iter.seek_to_first();
    while let tier = iter.next() {
        if tier.is_none() {
            break;
        }
        let re = tier.unwrap();
        println!("order {:?}", re.0);
    }
}
fn getorder(src: &order::Order) -> order::Order {
    return src.clone();
}
fn init_orders(mut input: &BPlusTree<order::Order, order::Order>) {
    input.insert(
        order::Order::new(
            "1001".to_string(),
            Decimal::from_str("127.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
        order::Order::new(
            "1001".to_string(),
            Decimal::from_str("127.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
    );
    input.insert(
        order::Order::new(
            "1002".to_string(),
            Decimal::from_str("129.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
        order::Order::new(
            "1002".to_string(),
            Decimal::from_str("129.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
    );
    input.insert(
        order::Order::new(
            "1003".to_string(),
            Decimal::from_str("117.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
        order::Order::new(
            "1003".to_string(),
            Decimal::from_str("117.129").unwrap(),
            Decimal::from_str("129.122").unwrap(),
        ),
    );
}
