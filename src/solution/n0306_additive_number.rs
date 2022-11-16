use super::Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let mut first = 0;
        let num_arr: Vec<char> = num.chars().collect();
        for i in 0..num.len() {
            if i > 0 && num_arr[0] == '0' {
                return false;
            }
            first = first * 10 + (num_arr[i] as u8 - '0' as u8) as i64;
            let mut second = 0;
            for j in i + 1..num.len() {
                second = second * 10 + (num_arr[j] as u8 - '0' as u8) as i64;
                if j > i + 1 && num_arr[i + 1] == '0' {
                    break;
                }
                if j + 1 < num.len() && is_can_added(first, second, num.as_str(), j + 1) {
                    return true;
                }
            }
        }
        false
    }
}

fn is_can_added(first: i64, second: i64, num: &str, sum_idx: usize) -> bool {
    if sum_idx == num.len() {
        return true;
    }

    let sum_str = i64::to_string(&(first + second));
    if sum_idx + sum_str.len() > num.len() {
        return false;
    }

    let actual_sum = &num[sum_idx..sum_idx + sum_str.len()];
    actual_sum == sum_str && is_can_added(second, first + second, num, sum_idx + sum_str.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0306() {
        assert_eq!(Solution::is_additive_number(String::from("112358")), true);
    }
}
