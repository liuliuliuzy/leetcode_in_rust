// derive 表示使用过程宏实现标准库的Eq等trait
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    // 允许多个指向某个节点的指针
    pub next: Option<Box<ListNode>>,
    // 这里的定义就是Option
    // 所以意思就是next可能指向一个节点，也可能为None。
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 打印单向链表，以头指针的不可变借用为参数
pub fn print_linked_list(head: &Option<Box<ListNode>>) {
    let mut p = head;
    // 这里会自动解引用然后借用？
    while let Some(node) = p {
        print!("{}->", node.val);
        p = &node.next;
    }
    println!();
}

// 将vector转换为链表
pub fn vec_to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

// pub fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {

// }
