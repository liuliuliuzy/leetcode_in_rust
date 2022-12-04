use super::Solution;

use std::collections::{hash_map::RandomState, HashSet};
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, mut topping_costs: Vec<i32>, target: i32) -> i32 {
        let (target, mut v, mut hs) = (
            target as i16,
            Vec::new(),
            HashSet::<i16, RandomState>::from_iter(base_costs.into_iter().map(|x| x as i16)), // from_iter is a trait (FromIterator)
        );
        let mut diff = hs.iter().map(|x| (target - x).abs()).min().unwrap();
        topping_costs.sort_unstable();
        topping_costs
            .into_iter()
            .rev() // from the max to min
            .map(|x| x as i16)
            .for_each(|x| {
                hs.iter().for_each(|&c| {
                    if c + 2 * x < target + diff {
                        v.push(c + x * 2);
                        v.push(c + x);
                        diff = diff
                            .min((target - (c + x * 2)).abs())
                            .min((target - (c + x)).abs())
                    } else if c + x < target + diff {
                        v.push(c + x);
                        diff = diff.min((target - (c + x)).abs())
                    }
                });
                hs.extend(v.drain(..))
            });
        hs.into_iter()
            .filter(|x| (target - x).abs() == diff)
            .min()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1774() {
        assert_eq!(Solution::closest_cost(vec![1, 7], vec![3, 4], 10), 10);
        assert_eq!(Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18), 17);
        assert_eq!(Solution::closest_cost(vec![3, 10], vec![2, 5], 9), 8);
        assert_eq!(Solution::closest_cost(vec![10], vec![1], 1), 10);
    }
}
