use super::Solution;
use std::cmp;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![s.len() as i32; s.len()];
        for (i, cha) in s.chars().enumerate() {
            if cha == c {
                ans[i] = 0;
                let mut left: i32 = i as i32 - 1;
                let mut right: i32 = i as i32 + 1;
                while left >= 0 && s.as_bytes()[left as usize] as char != c {
                    ans[left as usize] = cmp::min(ans[left as usize], i as i32 - left);
                    left -= 1;
                }
                while right < s.len() as i32 && s.as_bytes()[right as usize] as char != c {
                    ans[right as usize] = cmp::min(ans[right as usize], right - i as i32);
                    right += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0821() {
        assert_eq!(
            Solution::shortest_to_char(String::from("loveleetcode"), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
    }
}
