use super::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold::<(i32, i32), _>((0, 0), |(now, res), &x| (now + x, res.max(now + x)))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1732() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0)
    }
}
