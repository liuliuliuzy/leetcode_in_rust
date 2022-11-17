use super::Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        nums.iter()
            .enumerate()
            .all(|(i, &num)| i32::abs(num - i as i32) <= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0775() {
        assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
        assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
    }
}
