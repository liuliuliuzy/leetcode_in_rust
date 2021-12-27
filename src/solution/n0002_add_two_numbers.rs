pub struct Solution;

// 允许没有用到的import，消除告警
#[allow(unused_imports)]
use crate::data_structures::linked_list::{vec_to_linked_list, ListNode};

/*
给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/add-two-numbers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 这里直接赋值会发生 move 吗？
        let (mut l1, mut l2) = (l1, l2);
        // 创建头节点，定义一个指向头节点的dummy head
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        // 用于遍历的指针 tail
        let mut tail = &mut dummy_head;
        // 用于表示遍历边界的flag标志变量
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
        // 遍历链表
        loop {
            // 读取节点值
            let lhs = match l1 {
                // 如果不为None，那么就执行以下操作
                Some(node) => {
                    l1 = node.next;
                    node.val // 以node.val作为match块的返回结果
                }
                // 如果l1 已经遍历到头了，那么就返回一个假的节点值0
                None => {
                    l1_end = true;
                    0
                }
            };
            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            // 循环终止条件
            if l1_end && l2_end && !overflow {
                // 返回 dummy_head的next
                break dummy_head.unwrap().next;
            }
            let sum = lhs + rhs + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false; // 这里 overflow 要记得设为false，不然loop循环可能不会退出
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            // 移动节点指针
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}

// #[cfg(test)] 表示，只有在 cargo test test_0002的时候才会编译以下代码，cargo build的时候以下代码是不会被编译的
#[cfg(test)]
mod tests {
    // 因为当前处于模块 tests 内，所以需要use super
    use super::*;

    #[test]
    fn test_0002() {
        // 这里使用了to_linked_list, 为什么编译器还是提示unused import？
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_linked_list(vec![2, 4, 3]),
                vec_to_linked_list(vec![5, 6, 4])
            ),
            vec_to_linked_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(
                vec_to_linked_list(vec![9, 9, 9, 9]),
                vec_to_linked_list(vec![9, 9, 9, 9, 9, 9])
            ),
            vec_to_linked_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(vec_to_linked_list(vec![0]), vec_to_linked_list(vec![0])),
            vec_to_linked_list(vec![0])
        )
    }
}
