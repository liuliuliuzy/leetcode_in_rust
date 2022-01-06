pub struct Solution;
/*
给你一个字符串 path ，表示指向某一文件或目录的 Unix 风格 绝对路径 （以 '/' 开头），请你将其转化为更加简洁的规范路径。

在 Unix 风格的文件系统中，一个点（.）表示当前目录本身；此外，两个点 （..） 表示将目录切换到上一级（指向父目录）；两者都可以是复杂相对路径的组成部分。任意多个连续的斜杠（即，'//'）都被视为单个斜杠 '/' 。 对于此问题，任何其他格式的点（例如，'...'）均被视为文件/目录名称。

请注意，返回的 规范路径 必须遵循下述格式：

    始终以斜杠 '/' 开头。
    两个目录名之间必须只有一个斜杠 '/' 。
    最后一个目录名（如果存在）不能 以 '/' 结尾。
    此外，路径仅包含从根目录到目标文件或目录的路径上的目录（即，不含 '.' 或 '..'）。

返回简化后得到的 规范路径 。
 * */

// 关键思路：想到对path进行"/" split操作
impl Solution {
    pub fn simplify_path(path: String) -> String {
        // 是不是要用栈来模拟
        let mut stack: Vec<String> = Vec::new();
        // 逐字符处理
        let mut tmp_s: String = String::new();
        // 从索引1开始遍历
        for c in path.chars().skip(1) {
            match c {
                // 碰到一个 '/' 字符
                '/' => {
                    // 不进行操作，清除tmp_s
                    if tmp_s.eq(".") {
                        tmp_s.clear();
                        continue;
                    }
                    // 出栈
                    if tmp_s.eq("..") {
                        if !stack.is_empty() {
                            stack.pop();
                        }
                        tmp_s.clear();
                        continue;
                    }
                    // 压栈
                    if !tmp_s.is_empty() {
                        stack.push(tmp_s.clone());
                        tmp_s.clear();
                    }
                }
                _ => {
                    tmp_s.push(c);
                }
            }
        }
        // 对最后一个单词做处理
        if !tmp_s.is_empty() {
            if tmp_s.eq("..") && !stack.is_empty() {
                stack.pop();
            } else if tmp_s.eq(".") {
                tmp_s.clear();
            } else {
                if !tmp_s.eq("..") {
                    stack.push(tmp_s);
                }
            }
        }
        let mut ans: String = String::from("/");
        ans.push_str(&(stack.join("/"))[..]);
        ans
    }
    // 看看人家的，9行搞定，我tm写了快50行🤣
    pub fn simplify_path_ii(path: String) -> String {
        let mut queue = Vec::new();
        path.split("/").for_each(|level| {
            match level {
                "." | "" => (),
                ".." => { queue.pop(); }, // 空的也能pop吗，好像是的...
                _ => queue.push(level),
            }
        });
        "/".to_string() + &queue.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0071() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/a//b////c/d//././/..")),
            String::from("/a/b/c")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/..")),
            String::from("/")
        );
    }
}
