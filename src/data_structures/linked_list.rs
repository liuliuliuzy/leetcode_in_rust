// derive 表示继承标准库的Eq等trait
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
    // 这里的定义就是Option
    // 所以意思就是next可能指向一个节点，也可能为None。
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 帮助函数

// 将vector转换为链表
pub fn vec_to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    // 从最后一个元素开始创建节点
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

// pub fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    
// }
 