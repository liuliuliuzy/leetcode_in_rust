pub struct Solution;

/*
给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

子数组 是数组中的一个连续部分。

进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
 * */
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 动态规划
        let n = nums.len();
        let mut ans: i32 = nums[0];
        let mut sum: i32 = nums[0];
        for i in 1..n {
            if sum > 0 {
                sum = sum + nums[i];
            } else {
                sum = nums[i];
            }
            if sum > ans {
                ans = sum;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0053() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }
}
