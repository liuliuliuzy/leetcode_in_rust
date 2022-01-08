pub struct Solution;

/*
一条包含字母 A-Z 的消息通过以下映射进行了 编码 ：

'A' -> 1
'B' -> 2
...
'Z' -> 26

要 解码 已编码的消息，所有数字必须基于上述映射的方法，反向映射回字母（可能有多种方法）。例如，"11106" 可以映射为：

    "AAJF" ，将消息分组为 (1 1 10 6)
    "KJF" ，将消息分组为 (11 10 6)

注意，消息不能分组为  (1 11 06) ，因为 "06" 不能映射为 "F" ，这是由于 "6" 和 "06" 在映射中并不等价。

给你一个只含数字的 非空 字符串 s ，请计算并返回 解码 方法的 总数 。

题目数据保证答案肯定是一个 32 位 的整数。
 * */
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        assert!(s.len() >= 1);
        let s = s.as_str();

        fn is_decodable(s: &str) -> bool {
            assert!(s.len() == 1 || s.len() == 2);
            if s.len() == 1 {
                return s != "0";
            }
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            (first == '1') || (first == '2' && second.to_digit(10).unwrap() <= 6)
        }

        let mut a = is_decodable(&s[0..1]) as i32;
        let mut b = if s.len() >= 2 {
            (is_decodable(&s[0..1]) && is_decodable(&s[1..2])) as i32
                + is_decodable(&s[0..2]) as i32
        } else {
            a
        };
        for i in 2..s.len() {
            let mut t = 0;
            if is_decodable(&s[i..i + 1]) {
                t += b;
            }
            if is_decodable(&s[i - 1..i + 1]) {
                t += a;
            }

            a = b;
            b = t;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0091() {
        assert_eq!(Solution::num_decodings(String::from("12")), 2);
        assert_eq!(Solution::num_decodings(String::from("226")), 3);
    }
}
