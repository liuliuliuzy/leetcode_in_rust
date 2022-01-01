pub struct Solution;

impl Solution {
    // 这题就是实现二分
    // 但是与普通二分有点不一样
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = nums.len();
        let mut l: i32 = 0;
        let mut r: i32 = ans as i32 - 1;
        while l <= r {
            let mid = ((l + r) / 2) as usize;
            if target <= nums[mid] {
                ans = mid;
                r = mid as i32 - 1;
            } else {
                l = mid as i32 + 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0035() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }
}
