use super::Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let (mut first, mut second): (i32, i32) = (-1, -1);

        for c in s.chars() {
            match c {
                c if c >= '0' && c <= '9' => {
                    let n: i32 = c.to_digit(10).unwrap() as i32;
                    if first == -1 {
                        first = n;
                    } else if n > first {
                        second = second.max(first);
                        first = n;
                    } else if n < first {
                        second = second.max(n);
                    }
                }
                _ => {}
            };
        }
        if second == first {
            -1
        } else {
            second
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1796() {
        assert_eq!(Solution::second_highest(String::from("dfa12321afd")), 2);
        assert_eq!(Solution::second_highest(String::from("abc1111")), -1);
        assert_eq!(Solution::second_highest(String::from("ck0777")), 0);
    }
}
