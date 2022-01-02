pub struct Solution;

/*
列表 arr 由在范围 [1, n] 中的所有整数组成，并按严格递增排序。请你对 arr 应用下述算法：

    从左到右，删除第一个数字，然后每隔一个数字删除一个，直到到达列表末尾。
    重复上面的步骤，但这次是从右到左。也就是，删除最右侧的数字，然后剩下的数字每隔一个删除一个。
    不断重复这两步，从左到右和从右到左交替进行，直到只剩下一个数字。

给你整数 n ，返回 arr 最后剩下的数字。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/elimination-game
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * */
impl Solution {
    // 这nm是个数学题
    // 模拟肯定是过不了的
    pub fn last_remaining(n: i32) -> i32 {
        let mut remain = n;
        let mut left_to_right: bool = true;
        let mut ans = 1;
        let mut step = 1;
        // if there are still more numbers
        while remain > 1 {
            if left_to_right || remain % 2 == 1 {
                ans += step;
            }
            left_to_right = !left_to_right;
            step = step * 2;
            remain = remain / 2;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_0390() {
        assert_eq!(Solution::last_remaining(9), 6);
        assert_eq!(Solution::last_remaining(1), 1);
    }
}
