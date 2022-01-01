/*
给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。

最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。

你可以假设除了整数 0 之外，这个整数不会以零开头。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/plus-one
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 * */
pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        // check for https://doc.rust-lang.org/std/vec/struct.Vec.html for vector api
        let mut carry = 1;
        // digits是imutable，所以不能在 digits 的基础上改
        let mut ans: Vec<i32> = Vec::new();
        for var in digits.iter().rev() {
            // var 是borrow的i32类型，也就是 &i32
            let one = *var + carry;
            if one > 9 {
                carry = 1;
            } else {
                carry = 0;
            }
            ans.insert(0, one % 10);
        }
        if carry > 0 {
            ans.insert(0, 1);
        }
        ans
    }
}


#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_0061() {
        assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9,9,9,9]), vec![1,0,0,0,0]);
    }
}
