// derive 表示继承标准库的Eq等trait
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    // 允许多个指向某个节点的指针
    pub next: Option<Rc<RefCell<ListNode>>>,
    // 这里的定义就是Option
    // 所以意思就是next可能指向一个节点，也可能为None。
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 将vector转换为链表
pub fn vec_to_linked_list(vec: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut current = None;
    // 从最后一个元素开始创建节点
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Rc::new(RefCell::new(node)));
    }
    current
}

// pub fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {

// }
