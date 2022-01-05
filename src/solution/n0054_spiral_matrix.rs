use std::{collections::HashMap, usize, vec};

pub struct Solution;

#[allow(unused_comparisons)]
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // 模拟
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        // 哈希表记录
        let mut seen: HashMap<usize, bool> = HashMap::new();
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            for j in 0..n {
                seen.insert((i << 4) + j, false);
            }
        }
        // 用走过的步长来判断是否到达终点
        let mut step_length = 0;
        let mut ans: Vec<i32> = Vec::new();
        let (mut index_i, mut index_j) = (0, 0);
        let mut dir_index = 0;

        // println!("got hashmap: {:?}", &seen);
        // println!("hashmap[0 << 4 + 2] = {}", seen.get(&((0<<4) + 2)).unwrap());
        while step_length < m * n {
            // println!("actual: {}, {}", index_i, index_j);
            // 进入了循环，就代表还没走到头，所以当前点是有效的
            ans.push(matrix[index_i][index_j]);
            // 哈希表记录
            seen.insert((index_i << 4) + index_j, true);
            // println!("after: {:?}", seen);
            // 步长 + 1
            step_length += 1;

            // 先计算当前方向的下一步坐标，再判断其是否合法
            let next_i = (index_i as i32 + directions[dir_index].0) as usize;
            let next_j = (index_j as i32 + directions[dir_index].1) as usize;
            // println!("next  : {}, {}", next_i, next_j);
            if next_i < 0
                || next_i > m - 1
                || next_j < 0
                || next_j > n - 1
                || *(seen.get(&((next_i << 4) + next_j)).unwrap())
            {
                // println!("illegal!: \nnext_i->{}\nnext_j->{}\nkey->{}", next_i, next_j, ((next_i << 4) + next_j));
                // println!("value: {:?}", seen.get(&((next_i << 4) + next_j)));
                // 不合法的话，换个方向
                dir_index = (dir_index + 1) % 4;
            }
            // 在未走完所有格子之前，换个方向的下一步坐标必然是没走过的
            // 所以这里不需要判断
            index_i = (index_i as i32 + directions[dir_index].0) as usize;
            index_j = (index_j as i32 + directions[dir_index].1) as usize;
        }
        ans
    }
}

// 单元测试

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0054() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
