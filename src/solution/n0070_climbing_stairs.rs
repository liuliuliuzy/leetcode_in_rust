use super::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            1
        } else if n == 2 {
            2
        } else {
            let mut a = 1; // f(n-2)
            let mut b = 2; // f(n-1)
            for _ in 3..=n {
                let tmp = b + a;
                a = b;
                b = tmp;
            }
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0070() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(10), 89);
        assert_eq!(Solution::climb_stairs(20), 10946);
    }
}
