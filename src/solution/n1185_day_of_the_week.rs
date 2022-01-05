pub struct Solution;

/*
给你一个日期，请你设计一个算法来判断它是对应一周中的哪一天。

输入为三个整数：day、month 和 year，分别表示日、月、年。

您返回的结果必须是这几个值中的一个 {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}。
 * */

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let m = vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let d = vec![
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
        ];
        let mut days = 0;
        let mut y = 1971;

        // 这样不会耗费很多时间吗
        while year > y {
            days += 365;
            if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
                days += 1;
            }
            y += 1;
        }
        days += m[(month - 1) as usize] + day;
        if month > 2 && (y % 4 == 0 && y % 100 != 0 || y % 400 == 0) {
            days += 1;
        };
        d[(days % 7) as usize].to_string()
    }

    // 第二种方法：蔡勒公式
    // W = (day + 2 * month + 3 * (month + 1) / 5 + year + year / 4 - year / 100 + year / 400) mod 7
    pub fn day_of_the_week_ii(day: i32, mut month: i32, mut year: i32) -> String {
        let days = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        if month < 3 {
            month += 12;
            year -= 1;
        }

        days[((day + 2 * month + 3 * (month + 1) / 5 + year + year / 4 - year / 100
            + year / 400
            + 1)
            % 7) as usize]
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1185() {
        assert_eq!(
            Solution::day_of_the_week(31, 8, 2019),
            String::from("Saturday")
        );
        assert_eq!(
            Solution::day_of_the_week(18, 7, 1999),
            String::from("Sunday")
        );
        assert_eq!(
            Solution::day_of_the_week(15, 8, 1993),
            String::from("Sunday")
        );
    }
}
