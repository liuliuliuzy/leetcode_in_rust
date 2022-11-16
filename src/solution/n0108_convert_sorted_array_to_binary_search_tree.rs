#[allow(unused_imports)]
use crate::data_structures::tree_util::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

use super::Solution;
/*
给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。

高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * */
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 定义函数
        fn helper(nums: &Vec<i32>, p: usize, q: usize) -> Option<Rc<RefCell<TreeNode>>> {
            // 终止条件
            if p == q {
                return None;
            }
            let m = (p + q) / 2;
            let mut root = TreeNode::new(nums[m]);
            let left = helper(nums, p, m);
            let right = helper(nums, m + 1, q);
            root.left = left;
            root.right = right;
            return Some(Rc::new(RefCell::new(root)));
        }
        return helper(&nums, 0, nums.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0108() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
    }
}
