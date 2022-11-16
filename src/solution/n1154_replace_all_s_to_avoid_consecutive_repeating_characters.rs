use std::char;

use super::Solution;
/*
给你一个仅包含小写英文字母和 '?' 字符的字符串 s，请你将所有的 '?' 转换为若干小写字母，使最终的字符串不包含任何 连续重复 的字符。

注意：你 不能 修改非 '?' 字符。

题目测试用例保证 除 '?' 字符 之外，不存在连续重复的字符。

在完成所有转换（可能无需转换）后返回最终的字符串。如果有多个解决方案，请返回其中任何一个。可以证明，在给定的约束条件下，答案总是存在的。
 * */
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut res: Vec<char> = Vec::new();
        for c in s.chars() {
            res.push(c);
        }
        for i in 0..res.len() {
            if res[i] == '?' {
                let mut candidate: u8 = 0x61;
                if i > 0 && (candidate as char) == res[i - 1] {
                    candidate += 1;
                }
                if i < res.len() - 1 && (candidate as char) == res[i + 1] {
                    candidate += 1;
                    if i > 0 && (candidate as char) == res[i - 1] {
                        candidate += 1;
                    }
                }
                res[i] = candidate as char;
            }
        }
        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1154() {
        assert_eq!(
            Solution::modify_string(String::from("b?a")),
            String::from("bca")
        );
        assert_eq!(
            Solution::modify_string(String::from("ubv?w")),
            String::from("ubvaw")
        );
        assert_eq!(
            Solution::modify_string(String::from("j?qg??b")),
            String::from("jaqgacb")
        );
        assert_eq!(
            Solution::modify_string(String::from("??yw?ipkj?")),
            String::from("abywaipkja")
        );
    }
}
