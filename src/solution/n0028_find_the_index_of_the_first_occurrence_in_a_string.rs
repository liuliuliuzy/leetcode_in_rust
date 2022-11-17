use super::Solution;

/**
 * 给你两个字符串 haystack 和 needle ，
 * 请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
 *
 * 如果 needle 不是 haystack 的一部分，则返回  -1 。
 */

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // kmp in rust
        let (n, m) = (haystack.len(), needle.len());
        if n < m {
            return -1;
        }
        if m == 0 {
            return 0;
        }
        let mut pmt: Vec<usize> = vec![0; m];
        // preprocessing for needle: O(m)
        let mut j = 0;
        for i in 1..m {
            while j > 0 && needle.chars().nth(i).unwrap() != needle.chars().nth(j).unwrap() {
                j = pmt[j - 1];
            }
            if needle.chars().nth(i).unwrap() == needle.chars().nth(j).unwrap() {
                j += 1;
            }
            pmt[i] = j;
        }
        j = 0;
        // scan haystack only once: O(n)
        for i in 0..n {
            // got a mismatch with haystack[i] & needle[j]
            while j > 0 && haystack.chars().nth(i).unwrap() != needle.chars().nth(j).unwrap() {
                j = pmt[j - 1];
            }
            if haystack.chars().nth(i).unwrap() == needle.chars().nth(j).unwrap() {
                j += 1;
            }
            // matched all characters in needle, then return the value
            if j == m {
                return i as i32 - m as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n0028() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );

        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
