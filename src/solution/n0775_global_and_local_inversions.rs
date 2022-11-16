use super::Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        nums.iter()
            .enumerate()
            .all(|(i, &num)| i32::abs(num - i as i32) <= 1)
    }
}
