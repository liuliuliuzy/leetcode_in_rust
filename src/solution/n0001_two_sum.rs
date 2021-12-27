pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        // 遍历数组元素
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                // 如果记录不存在的话，往哈希表中插入记录
                None => {
                    map.insert(num, index);
                }
                // 如果存在的话，那么target - num的索引和当前的index就组成了我们要的答案
                // 因为Some的match产生的结果是&usize类型，所以这里需要 * 解引用。
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0001() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
