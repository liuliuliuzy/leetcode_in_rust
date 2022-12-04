use super::Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut count = -1;
        let mut min = i32::MAX;
        for (index, ele) in points.iter().enumerate() {
            if ele[0] == x || ele[1] == y {
                let d = (ele[0] - x).abs() + (ele[1] - y).abs();
                if d < min {
                    min = d;
                    count = index as i32;
                }
            }
        }
        count
    }
}
