mod bp_test;
// 定义链表节点
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

// 定义链表
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // 在链表头部插入元素
    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    // 移动指针到下一个节点
    fn move_to_next(&mut self) {
        if let Some(node) = self.head.as_mut() {
            self.head = node.next.take();
        }
    }
}
#[test]
fn test_node() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    // 移动指针到下一个节点
    // list.move_to_next();
    // list.move_to_next();
    // 打印当前节点的值
    if let Some(node) = &list.head {
        println!("当前节点的值: {:?}", node.value);
    } else {
        println!("链表为空。");
    }
}
use std::ops::Deref;

use crate::core::node;
#[test]
fn test_mynode() {
    let head = node::Node::new("hello");
    let newnode = node::Node::new2("node2", head);
    let mut tnode = node::Node::new2("node3", newnode);
    print_all_node(tnode.clone());
    //原生
    if let Some(vnode) = tnode.as_mut() {
        let mut vv = vnode._next.as_mut();
        if let Some(mut vvnode) = vv {
            vvnode._value = "i change it";
        }
    }
    print_all_node(tnode.clone());
}
// 封装后
#[test]
fn test_listnode() {
    use node::interfaces::Node;
    let mut nd = node::ListNode::new("one");
    let mut nd2 = node::ListNode::new("two");
    let mut nd3 = node::ListNode::new("three");

    nd.set_next(nd2, 1);
    nd.set_next(nd3, 2);
    let mut nd2ref = nd.next_ref(1);
    nd2ref.set_val("three-en", 0);
    nd2ref.set_val("four", 1);
    print_all_listnode(nd);
    // nd.next();
}
fn print_all_node(mut _head: Option<Box<node::Node<&str>>>) {
    let mut i = 0;
    let mut ee = _head.as_mut();
    while let Some(mut node2) = _head {
        i += 1;
        println!("{i} {}", node2._value);
        _head = node2._next.take();
    }
}
fn print_all_listnode(mut _head: node::ListNode<&str>) {
    let mut i = 0;
    while !_head.is_none() {
        i += 1;
        println!("{i} {}", _head.getval().unwrap());
        _head = _head.next();
    }
}

#[test]
fn test_std_list() {
    use std::collections::LinkedList;
    let mut head: LinkedList<u64> = LinkedList::new();
    head.push_back(23);
    head.push_back(10);
    head.push_back(5);
    let nodes = head.back();
    let mut i = head.iter();
    i.next();
    println!("node value {}", nodes.unwrap());
}

#[test]
fn test_collection() {}
