use super::Solution;

const MOD_L: u64 = 1e9 as u64 + 7;

impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let (mut res, mut x, mut y): (u64, u64, u64) = (0, nums[0] as u64, 2);
        let n = nums.len();
        for i in 1..n {
            res = (res + nums[i] as u64 * (y - 1) - x) % MOD_L;
            x = (x * 2 + nums[i] as u64) % MOD_L;
            y = y * 2 % MOD_L;
        }
        (res % MOD_L) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0891() {
        assert_eq!(Solution::sum_subseq_widths(vec![2, 1, 3]), 6);
        assert_eq!(Solution::sum_subseq_widths(vec![2]), 0);
        assert_eq!(
            Solution::sum_subseq_widths(vec![
                5, 69, 89, 92, 31, 16, 25, 45, 63, 40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92,
                44, 67, 38, 92, 22, 91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95, 76, 84, 79, 89, 4,
                16, 94, 16, 77, 92, 9, 30, 13
            ]),
            857876214
        );
    }
}
