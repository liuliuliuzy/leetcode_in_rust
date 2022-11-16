use super::Solution;
use std::cmp;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // 遍历所有路径的话，需要C(m, n)的时间复杂度
        // 动态规划
        let (m, n) = (grid.len(), grid[0].len());
        // 初始化一个 m * n 的dp矩阵
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let right = {
                    if j + 1 < n {
                        dp[i][j + 1]
                    } else {
                        -1
                    }
                };
                let down = {
                    if i + 1 < m {
                        dp[i + 1][j]
                    } else {
                        -1
                    }
                };
                if right >= 0 && down >= 0 {
                    dp[i][j] = cmp::min(right, down) + grid[i][j];
                } else {
                    dp[i][j] = grid[i][j] + {
                        if right >= 0 {
                            right
                        } else if down >= 0 {
                            down
                        } else {
                            0
                        }
                    }
                }
                // println!("dp: {:?}", &dp);
            }
        }
        // println!("dp: {:?}", &dp);
        dp[0][0]
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_0064() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
