use super::Solution;

/**
 * 通过简单题来练习Rust
给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。

不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。

提示：
    0 <= nums.length <= 3 * 104
    -104 <= nums[i] <= 104
    nums 已按升序排列

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            // 为什么这里一定要用return语句，不能用表达式？
            return len as i32;
        }
        let mut slow: usize = 0;
        let mut fast: usize = 1;
        while fast < len {
            if nums[fast] != nums[slow] {
                slow += 1;
                // 记录一下：因为nums是&mut 类型，所以这里可以通过nums变量直接修改数组中的元素
                nums[slow] = nums[fast];
            }
            fast += 1;
        }
        // 从头开始截取slow+1长度的元素
        nums.truncate(slow + 1);
        (slow + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0026() {
        assert_eq!(Solution::remove_duplicates(&mut (vec![1, 2, 2, 3])), 3);
        assert_eq!(Solution::remove_duplicates(&mut (vec![1, 1, 2])), 2);
        assert_eq!(
            Solution::remove_duplicates(&mut (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])),
            5
        );
    }
}
