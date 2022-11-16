use super::Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut prod = 1;

        // 方法1：枚举右边界，滑动左边界
        // let mut i = 0;
        // for j in 0..nums.len() {
        //     prod = prod * nums[j];
        //     while i <= j && prod >= k {
        //         // 将i左移1位，当前结果除以nums[i]
        //         prod = prod / nums[i];
        //         i += 1;
        //     }
        //     ans += j - i + 1;
        // }

        // 方法2：枚举左边界，滑动右边界
        let mut j = 0;
        for i in 0..nums.len() {
            // 循环体中每次变化的是i
            while j < nums.len() && prod < k {
                prod *= nums[j];
                j += 1;
            }
            // 情况要分类讨论
            if prod >= k && j > i {
                ans += j - i - 1;
            } else {
                if j >= i {
                    ans += j - i;
                }
            }
            println!("{}-{}-{}", i, j, ans);
            prod /= nums[i];
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0713_case1() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 7),
            6
        );
    }

    #[test]
    fn test_0713_case2() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }
}
