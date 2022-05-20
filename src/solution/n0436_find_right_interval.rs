pub struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pii(i32, i32);

fn binary_search(piis: &Vec<Pii>, target: i32) -> i32 {
    let (mut l, mut r) = (0, (piis.len() - 1) as i32);
    if target < piis[l as usize].0 || target > piis[r as usize].0 {
        return -1;
    }
    let mut m;
    while l < r {
        m = l + ((r - l) >> 1);
        if piis[m as usize].0 < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    piis[l as usize].1
}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut ans = vec![0; n];
        let mut lefts = vec![];
        for i in 0..n {
            lefts.push(Pii(intervals[i as usize][0], i as i32)); // (start_i, i)
        }

        lefts.sort();
        for i in 0..n {
            ans[i] = binary_search(&lefts, intervals[i][1]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0436() {
        assert_eq!(
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
            vec![-1, 2, -1]
        );
    }
}
