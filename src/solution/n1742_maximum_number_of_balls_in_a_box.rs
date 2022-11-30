use super::Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut cnts = [0; 46]; // idx: 1 ~ 45
        (low_limit..=high_limit).fold(0, |maximum, i| {
            let (mut tmp, mut sum) = (i, 0);
            while tmp > 0 {
                sum += tmp % 10;
                tmp /= 10;
            }
            cnts[sum as usize] += 1;
            maximum.max(cnts[sum as usize])
        })
    }
}
