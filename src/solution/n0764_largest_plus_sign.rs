use super::Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let dirs: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut g = vec![vec![1; n as usize]; n as usize];
        mines
            .iter()
            .for_each(|m| g[m[0] as usize][m[1] as usize] = 1);
        let mut ans = 0;
        (0..n as usize).for_each(|i| {
            (0..n as usize).for_each(|j| {
                if g[i][j] == 1 {
                    let mut k = 0;
                    while dirs
                        .iter()
                        .all(|dir| g[i + k * dir[0] as usize][j + k * dir[1] as usize] == 1)
                    {
                        k += 1;
                    }
                    ans = ans.max(k + 1);
                }
            })
        });
        ans as i32
    }
}
