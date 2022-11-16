use super::Solution;

/*
给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/majority-element
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

进阶：

    尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。
 * */

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for x in nums.iter() {
            // 当值不存在的时候插入0
            let value = counts.entry(*x).or_insert(0);
            *value += 1;
        }
        let mut ans: i32 = 0;
        // iterate HashMap
        for (number, count) in &counts {
            if *count >= (nums.len() + 1) / 2 {
                ans = *number;
                break;
            }
        }
        ans
    }

    pub fn majority_element_in_boyer_moore_method(nums: Vec<i32>) -> i32 {
        let mut candidate = nums[0];
        let mut count = 1;
        // skip first element when iterating
        for num in nums.iter().skip(1).copied() {
            if count == 0 {
                candidate = num;
            }
            if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0169() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_0169_moore() {
        assert_eq!(
            Solution::majority_element_in_boyer_moore_method(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        );
        assert_eq!(
            Solution::majority_element_in_boyer_moore_method(vec![3, 2, 3]),
            3
        );
    }
}
