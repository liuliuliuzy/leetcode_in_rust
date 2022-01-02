use std::{usize, vec};

pub struct Solution;

/**
给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。
 */
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // 不用递归的话
        let mut ans: Vec<Vec<i32>> = Vec::new();
        // 初始状态
        ans.push(vec![1]);
        for i in 1..num_rows {
            let mut tmp: Vec<i32> = vec![1];
            if ans[(i - 1) as usize].len() > 1 {
                for j in 0..i - 1 {
                    tmp.push(
                        ans[(i - 1) as usize][j as usize] + ans[(i - 1) as usize][(j + 1) as usize],
                    );
                }
            }
            tmp.push(1);
            ans.push(tmp);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
        );
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
