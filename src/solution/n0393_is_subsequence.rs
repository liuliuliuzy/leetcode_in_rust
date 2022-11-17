use super::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (m, n) = (s.len(), t.len());
        let (mut i, mut j) = (0 as usize, 0 as usize);
        while i < m && j < n {
            if s.chars().nth(i).unwrap() == t.chars().nth(j).unwrap() {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == m
    }

    // 超时。。。
    pub fn is_subsequence_v2(s: String, t: String) -> bool {
        // dp
        let mut t = t;
        t.insert(0, ' ');
        let n = t.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 26]; n]; // dp[i][j]表示，在t的i位置上往后看，下一个字符出现的位置，字符为'a'+j
        for c in 'a'..='z' {
            let mut v = -1;
            for i in (0..=(n - 1)).into_iter().rev() {
                dp[i][c as usize - 'a' as usize] = v;
                if t.chars().nth(i).unwrap() == c {
                    v = i as i32;
                }
            }
        }
        let mut i = 0;
        for c in s.chars() {
            i = dp[i as usize][c as usize - 'a' as usize];
            if i == -1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0792() {
        assert_eq!(
            Solution::is_subsequence_v2(String::from("abc"), String::from("ahbgdc")),
            true
        );

        assert_eq!(
            Solution::is_subsequence_v2(String::from("axc"), String::from("ahbgdc")),
            false
        );

        assert_eq!(
            Solution::is_subsequence_v2(String::from(""), String::from("frr")),
            true
        );
    }
}
