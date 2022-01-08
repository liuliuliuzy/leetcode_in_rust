pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..(1 << n)).map(|i| i ^ i >> 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0089() {
        assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
