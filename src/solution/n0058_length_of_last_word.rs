use super::Solution;

/*
给你一个字符串 s，由若干单词组成，单词前后用一些空格字符隔开。返回字符串中最后一个单词的长度。

单词 是指仅由字母组成、不包含任何空格字符的最大子字符串。
 * */
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_0058() {
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
        // assert_eq!(Solution::length_of_last_word(String::from(" ")), 0);
    }
}
