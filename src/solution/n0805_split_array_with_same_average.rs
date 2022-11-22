use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        // 特殊情况
        if n == 1 {
            return false;
        }

        let m = n / 2;

        // preprocessing
        let mut nums = nums;
        let sum: i32 = nums.iter().sum();
        nums.iter_mut().for_each(|num| *num = *num * n as i32 - sum);

        // hashset需要可变
        let mut left: HashSet<i32> = HashSet::new();
        for i in 1..(1 << m) {
            // 二进制枚举，计算和
            let mut total = 0;
            for j in 0..m {
                if i & (1 << j) > 0 {
                    total += nums[j];
                }
            }
            if total == 0 {
                return true;
            }
            left.insert(total);
        }

        let rsum: i32 = nums[m..n].iter().sum();
        for i in 1..(1 << (n - m)) {
            let mut total = 0;
            for j in m..n {
                if i & (1 << (j - m)) > 0 {
                    total += nums[j];
                }
            }
            if total == 0 || (total != rsum && left.contains(&-total)) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0805() {
        assert_eq!(
            Solution::split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            true
        );
        assert_eq!(Solution::split_array_same_average(vec![3, 1]), false);
        assert_eq!(Solution::split_array_same_average(vec![0]), false);
    }
}
