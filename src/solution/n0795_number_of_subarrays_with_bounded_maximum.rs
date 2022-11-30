use super::Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let (mut l1, mut l2): (i32, i32) = (-1, -1);
        nums.iter().enumerate().fold(0, |res, (i, &n)| {
            match n {
                _ if n >= left && n <= right => {
                    l1 = i as i32;
                }
                _ if n > right => {
                    l2 = i as i32;
                    l1 = -1;
                }
                _ => {}
            }
            if l1 != -1 {
                res + l1 - l2
            } else {
                res
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0795() {
        assert_eq!(
            Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3),
            3
        );

        assert_eq!(
            Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8),
            7
        );
    }
}
