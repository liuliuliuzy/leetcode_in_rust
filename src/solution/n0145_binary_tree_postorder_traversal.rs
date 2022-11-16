use super::Solution;
use crate::data_structures::tree_util::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    // 任务：使用Rust实现迭代版的二叉树后序遍历
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        // 当前节点初始化为根节点
        let mut current_root_wrap = root;
        // 如果当前节点不为None或栈不为空
        while current_root_wrap.is_some() || !stack.is_empty() {
            while let Some(node) = current_root_wrap {
                current_root_wrap = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                if node.borrow().right.is_some() {
                    current_root_wrap = node.borrow_mut().right.take();
                    stack.push(node);
                } else {
                    ans.push(node.borrow().val);
                }
            }
        }
        ans
    }
}
