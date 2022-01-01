pub struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if original.len() != (m * n) as usize {
            // 返回空的二维数组
            return ans;
        }
        for i in 0..m {
            let mut t: Vec<i32> = Vec::new();
            for j in 0..n {
                t.push(original[(i * n + j) as usize]);
            }
            ans.push(t);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2022() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4, 5], 2, 2),
            vec![] as Vec<Vec<i32>>
        );
    }
}
