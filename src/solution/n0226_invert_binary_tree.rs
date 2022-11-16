use std::{cell::RefCell, rc::Rc};

use crate::data_structures::tree_util::{TreeNode, TreeNodeSwap};

use super::Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // if let Some 尝试捕获Option<T>中的值
        // 调用clone()，这样就不会获取所有权，所以后面还可以返回root
        if let Some(node) = root.clone() {
            // 右边的node是指向根节点的智能指针，borrow_mut()获取指针指向的值的可变引用，这样就可以借此修改节点的值
            let mut node = node.borrow_mut();
            // 递归调用，反转其左右子树
            let (left, right) = (
                // 调用clone()，不获取所有权
                Self::invert_tree(node.left.clone()),
                Self::invert_tree(node.right.clone()),
            );
            // node此时是节点的可变引用，RefMut<TreeNode>类型，所以可以修改节点的值。这里的`.`运算符会自动解引用
            node.left = right;
            node.right = left;
        }
        root
    }

    // iterative
    pub fn invert_tree_ii(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        // 根节点入栈
        root.as_ref().map(|node| stack.push(Rc::clone(node)));

        while let Some(ref node) = stack.pop() {
            node.borrow()
                .left
                .as_ref()
                .map(|node| stack.push(Rc::clone(node)));
            node.borrow()
                .right
                .as_ref()
                .map(|node| stack.push(Rc::clone(node)));
            node.borrow_mut().swap();
        }
        root
    }
}
