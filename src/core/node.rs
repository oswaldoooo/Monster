pub mod interfaces {
    pub trait Node<T>
    where
        T: Clone,
    {
        //panic if out of range
        fn set_val(&mut self, _val: T, _step: usize);
        //panic if out of range
        fn get_val(&self, _step: usize) -> T;
        //panic if out of range
        fn move_to(&mut self, _step: usize);
    }
}
pub struct Node<T> {
    pub _value: T,
    pub _next: Option<Box<Self>>,
}
impl<T> Clone for Node<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            _value: self._value.clone(),
            _next: self._next.clone(),
        }
    }
    // fn clone(&self) -> Self {
    //     return Self {
    //         _value: (),
    //         _next: (),
    //     };
    // }
}
impl<T: Clone> Node<T> {
    pub fn new(_value: T) -> Option<Box<Self>> {
        return Some(Box::new(Self {
            _value: _value,
            _next: None,
        }));
    }
    pub fn new2(_value: T, _next: Option<Box<Self>>) -> Option<Box<Self>> {
        return Some(Box::new(Self {
            _value: _value,
            _next: _next,
        }));
    }
    pub fn next(&mut self) -> Option<Box<Node<T>>> {
        return self._next.clone();
    }
    pub fn next_ref(&mut self) -> Option<&Box<Self>> {
        if let Some(mut vnode) = self._next.as_ref() {
            return Some(vnode);
        }
        return None;
    }
    //后移指定位
    pub fn go_to(mut _head: Option<Box<Node<T>>>, _pos: usize) -> Option<Box<Node<T>>> {
        let mut next: Option<Box<Node<T>>> = None;
        for i in [0.._pos] {
            if _head.is_none() {
                break;
            }
            next = _head.clone().unwrap()._next;
            _head = next.clone();
        }
        return next;
    }
}
#[derive(Clone)]
pub struct ListNode<T> {
    _node: Option<Box<Node<T>>>, //原始节点
}
impl<T> ListNode<T>
where
    T: Clone,
{
    pub fn new(_value: T) -> Self {
        return Self {
            _node: Node::new(_value),
        };
    }
    pub fn next(&self) -> Self {
        if let Some(vnode) = self._node.as_ref() {
            return Self {
                _node: vnode._next.clone(),
            };
        }
        return Self { _node: None };
    }
    pub fn setval(&mut self, _val: T) {
        if let Some(mut vnode) = self._node.as_mut() {
            vnode._value = _val;
        } else {
            panic!("node not inited");
        }
    }
    pub fn set_next(&mut self, mut _next: Self, _step: usize) {
        let mut head = self._node.as_mut();
        let mut i: usize = 0;
        while i < _step {
            if let Some(mut vnode) = head {
                if i == _step - 1 {
                    vnode._next = _next._node.take();
                    return;
                }
                head = vnode._next.as_mut();
                i += 1;
            } else {
                panic!("out of range");
            }
        }
    }
    pub fn next_ref(&mut self, _step: usize) -> ListNodeRef<'_, T> {
        let mut head: Option<&mut Box<Node<T>>> = self._node.as_mut();
        let mut i: usize = 0;
        while i <= _step {
            if let Some(mut vnode) = head {
                if i == _step {
                    // return ;
                    return ListNodeRef::new(vnode);
                }
                head = vnode._next.as_mut();
                // head = &ListNode { _node: vnode._next };
                i += 1;
            } else {
                panic!("out of range");
            }
        }
        return ListNodeRef::None();
    }
    pub fn getval(&self) -> Option<T> {
        if let Some(vnode) = self._node.clone() {
            return Some(vnode._value);
        }
        return None;
    }
    pub fn is_none(&self) -> bool {
        return self._node.is_none();
    }
}
//节点指针,指向链表节点
pub struct ListNodeRef<'a, T: Clone> {
    _node: Option<&'a mut Box<Node<T>>>,
}
impl<'a, T: Clone> ListNodeRef<'a, T> {
    pub fn new(_val: &'a mut Box<Node<T>>) -> Self {
        return Self { _node: Some(_val) };
    }
    pub fn None() -> Self {
        return Self { _node: None };
    }
    pub fn clone(&mut self) -> Self {
        let mut raw = self._node.take().unwrap();
        let refa = raw;
        // let refb = raw;
        return Self::None();
    }
    //向后移动一位
    pub fn next(&mut self) -> Self {
        // if let Some(vnode) = self._node.as_deref_mut() {
        //     return Self::new(vnode);
        // }
        if let Some(vnode) = self._node.take() {
            // self._node = vnode._next.as_mut();
            // self._node = Some(vnode);
            return Self {
                _node: vnode._next.as_mut(),
            };
        } else {
            panic!("out of range!");
        }
    }
}
impl<T> interfaces::Node<T> for ListNode<T>
where
    T: Clone,
{
    fn set_val(&mut self, _val: T, _step: usize) {
        let mut head = self._node.as_mut();
        let mut i: usize = 0;
        while i <= _step {
            if let Some(mut vnode) = head {
                if i == _step {
                    vnode._value = _val;
                    return;
                }
                i += 1;
                head = vnode._next.as_mut();
            } else {
                break;
            }
        }
    }

    fn get_val(&self, _step: usize) -> T {
        let mut head = self._node.as_ref();
        let mut i: usize = 0;
        while i <= _step {
            if let Some(vnode) = head {
                if i == _step {
                    return vnode._value.clone();
                }
                i += 1;
                head = vnode._next.as_ref();
            } else {
                break;
            }
        }
        panic!("out of range");
    }

    fn move_to(&mut self, _step: usize) {
        todo!()
    }
}
// impl<'a, T: Clone> Clone for ListNodeRef<'a, T> {
//     fn clone(&self) -> Self {}
// }
impl<'a, T> interfaces::Node<T> for ListNodeRef<'a, T>
where
    T: Clone,
{
    fn set_val(&mut self, _val: T, _step: usize) {
        let mut head: Option<&mut Box<Node<T>>> = self._node.as_deref_mut();

        // self._node = Some(ee);
        let mut i: usize = 0;
        while i <= _step {
            if let Some(vnode_head) = head {
                if i == _step {
                    unsafe {
                        vnode_head._value = _val;
                    }
                    return;
                }
                head = vnode_head._next.as_mut();
                i += 1;
            } else {
                panic!("out of range {i}");
            }
        }
    }
    fn get_val(&self, _step: usize) -> T {
        let mut head = self._node.as_deref();
        let mut i: usize = 0;
        while i <= _step {
            if let Some(vnode) = head {
                if i == _step {
                    return vnode._value.clone();
                }
                i += 1;
                head = vnode._next.as_ref();
            } else {
                panic!("out of range");
            }
        }
        panic!("out of range");
    }

    fn move_to(&mut self, _step: usize) {
        for i in [0.._step] {
            self.next();
        }
    }
}
