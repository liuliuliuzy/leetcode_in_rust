use super::Solution;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        // 参考 https://leetcode-cn.com/problems/reorder-data-in-log-files/solution/by-fffzlfk-vx95/
        // Rust写起来竟能如此之简洁？
        let mut logs = logs;
        logs.sort_by_key(|log| {
            let mut parts = log.splitn(2, ' ');
            // 因为是做题，所以不会出现异常数据，unwrap()这个遇到错误会panic的函数可以大胆地使用
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            if second.chars().next().unwrap().is_alphabetic() {
                // 返回3个元素的tuple，按照这3个元素自动升序排列
                (0, second.to_owned(), first.to_owned())
            } else {
                (1, "".to_owned(), "".to_owned())
            }
        });
        logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0937() {
        assert_eq!(
            Solution::reorder_log_files(vec![
                String::from("dig1 8 1 5 1"),
                String::from("let1 art can"),
                String::from("dig2 3 6"),
                String::from("let2 own kit dig"),
                String::from("let3 art zero")
            ]),
            vec![
                String::from("let1 art can"),
                String::from("let3 art zero"),
                String::from("let2 own kit dig"),
                String::from("dig1 8 1 5 1"),
                String::from("dig2 3 6")
            ]
        )
    }
}
