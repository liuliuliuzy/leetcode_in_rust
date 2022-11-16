use super::Solution;
use crate::data_structures::linked_list::ListNode;
use std::mem;

impl Solution {
    // 指向节点的指针类型为 Option<Box<ListNode>，所以每个节点最多只有一个可变引用或者都是不可变引用。
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 如果链表为空，直接返回
        if head.is_none() {
            return head;
        }
        // 覆盖绑定，改变head的可变性
        let mut head = head;
        let mut node = head.as_mut().unwrap();
        let mut duplicate = node.val;
        // replace(a, b)，将a中原本的值返回，a接收b的值
        while let Some(next) = mem::replace(&mut node.next, None) {
            if next.val == duplicate {
                node.next = next.next;
            } else {
                // 记录新的节点值
                duplicate = next.val;
                // 连接链表
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }
        }
        // 返回链表
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::linked_list::{print_linked_list, vec_to_linked_list};

    #[test]
    fn test_0083() {
        let l = vec_to_linked_list(vec![1, 1, 1, 2, 3]);
        print_linked_list(&l);
        assert_eq!(
            Solution::delete_duplicates(l),
            vec_to_linked_list(vec![1, 2, 3])
        );
    }
}
