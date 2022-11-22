use super::Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut row = vec![poured as f64];

        for i in 1..=query_row {
            let mut next_row = vec![0.0; i as usize + 1];
            for j in 0..row.len() {
                let volume = row[j];
                if volume > 1.0 {
                    next_row[j] += (volume - 1.0) / 2.0;
                    next_row[j + 1] += (volume - 1.0) / 2.0;
                }
            }
            row = next_row;
        }

        1.0_f64.min(row[query_glass as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0799() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
        assert_eq!(Solution::champagne_tower(6, 3, 2), 0.25);
    }
}
