use super::Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        left[0] = 0;
        right[n - 1] = 0;

        for i in 0..n - 1 {
            left[i + 1] = left[i];
            if boxes.chars().nth(i).unwrap() == '1' {
                left[i + 1] += 1;
            }
            right[n - 2 - i] = right[n - 1 - i];
            if boxes.chars().nth(n - 1 - i).unwrap() == '1' {
                right[n - 2 - i] += 1;
            }
        }

        let mut res = vec![0_i32; n];
        for i in 1..n {
            if boxes.chars().nth(i).unwrap() == '1' {
                res[0] += i as i32;
            }
        }

        for i in 1..n {
            res[i] = res[i - 1] - right[i] + left[i - 1];
            if boxes.chars().nth(i).unwrap() == '1' {
                res[i] -= 1;
            }
            if boxes.chars().nth(i - 1).unwrap() == '1' {
                res[i] += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1769() {
        assert_eq!(Solution::min_operations(String::from("110")), vec![1, 1, 3]);
        assert_eq!(
            Solution::min_operations(String::from("001011")),
            vec![11, 8, 5, 4, 3, 4]
        );
    }
}
