use super::Solution;

// 计算杨辉三角
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 数学方法
        let mut ans: Vec<i32> = Vec::with_capacity((row_index + 1) as usize);
        ans.push(1);
        for i in 1..row_index + 1 {
            if i > 1 {
                let mut tmp_value = ans[0];
                for j in 1..i {
                    let tmp = ans[j as usize];
                    ans[j as usize] = tmp_value + ans[j as usize];
                    tmp_value = tmp;
                }
            }
            ans.push(1);
        }
        // ans[0] = 1;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0119() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(
            Solution::get_row(10),
            vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1]
        );
    }
}
