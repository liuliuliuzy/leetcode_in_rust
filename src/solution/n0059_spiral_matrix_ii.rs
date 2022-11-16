use super::Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_index: usize = 0;
        let mut now_i = 0;
        let mut now_j = 0;
        for i in 1..n * n + 1 {
            ans[now_i][now_j] = i;
            let tmp_i = (now_i as i32 + directions[dir_index].0) as i32;
            let tmp_j = (now_j as i32 + directions[dir_index].1) as i32;
            if tmp_i < 0
                || tmp_i >= n
                || tmp_j < 0
                || tmp_j >= n
                || ans[tmp_i as usize][tmp_j as usize] > 0
            {
                dir_index = (dir_index + 1) % 4;
            }
            now_i = (now_i as i32 + directions[dir_index].0) as usize;
            now_j = (now_j as i32 + directions[dir_index].1) as usize;
        }
        ans
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_0059() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
