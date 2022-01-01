pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut slow: usize = 0;
        let mut index: usize = 0;
        while index < len {
            if nums[index] != val {
                nums[slow] = nums[index];
                slow += 1;
            }
            index += 1;
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0027() {
        assert_eq!(
            Solution::remove_element(&mut (vec![2, 3, 3, 1, 5, 3]), 3),
            3
        );
    }
}
