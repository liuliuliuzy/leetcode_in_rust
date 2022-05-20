pub struct RecentCounter {
    pings: Vec<i32>,
    cnts: i32,
}

impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            pings: vec![],
            cnts: 0,
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        // 二分法寻找区间之外的要被去除的请求数目
        if t > 3000 {
            let index = self.binary_search(t - 3000);
            println!("index: {}", index);
            if index > 0 {
                self.cnts -= index;
                self.pings.reverse();
                self.pings.truncate(self.pings.len() - index as usize);
                self.pings.reverse();
            }
        }
        self.pings.push(t);
        self.cnts += 1;
        self.cnts
    }

    fn binary_search(&self, target: i32) -> i32 {
        // 二分：寻找右区间的左边界
        let mut left: i32 = 0;
        let mut right: i32 = self.pings.len() as i32;

        while left < right {
            let mid = left + ((right - left) >> 1);
            if self.pings[mid as usize] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0933() {
        let mut obj = RecentCounter::new();
        let ret_1: i32 = obj.ping(1);
        assert_eq!(ret_1, 1, "wrong req_1");

        let ret_2: i32 = obj.ping(100);
        assert_eq!(ret_2, 2, "wrong req_2");

        let ret_3: i32 = obj.ping(3001);
        assert_eq!(ret_3, 3, "wrong req_3");

        let ret_4: i32 = obj.ping(3002);
        assert_eq!(ret_4, 3, "wrong req_4");
    }
}
