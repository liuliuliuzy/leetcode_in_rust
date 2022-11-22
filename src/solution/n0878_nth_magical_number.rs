use super::Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            b
        } else {
            Solution::gcd(b, a % b)
        }
    }
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const MOD_L: i64 = 1e9 as i64 + 7;

        let lcm = (a * b / Solution::gcd(a, b)) as i64;
        let (a, b) = (a as i64, b as i64);
        let (mut l, mut r) = (a.min(b) as i64, n as i64 * a.min(b) as i64);
        while l < r {
            let m = l + ((r - l) >> 1);
            let cnt = m / a + m / b - m / lcm;
            if cnt >= n as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        (l % MOD_L) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0878() {
        assert_eq!(Solution::nth_magical_number(1, 2, 3), 2);
        assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
    }
}
