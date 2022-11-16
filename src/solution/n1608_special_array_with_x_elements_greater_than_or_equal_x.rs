use super::Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        println!("{:?}", nums);
        let n = nums.len();
        if nums[0] >= n as i32 {
            return n as i32;
        }
        if nums[nums.len() - 1] < 0 {
            return 0;
        }
        for (i, _) in nums.iter().enumerate() {
            if i == 0 {
                continue;
            }
            println!("{} {} {}", i, nums[i], nums[i - 1]);
            if n as i32 - i as i32 <= nums[i] && n as i32 - i as i32 > nums[i - 1] {
                return n as i32 - i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1608() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
