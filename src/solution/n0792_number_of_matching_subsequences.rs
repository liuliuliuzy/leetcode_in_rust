use super::Solution;

#[derive(Clone)]
struct Pair {
    curr: String,
    idx: i32,
}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let (mut queue, mut cnt) = (vec![VecDeque::new(); 26], 0);
        for word in words {
            queue[word.as_bytes()[0] as usize - b'a' as usize]
                .push_back(Pair { curr: word, idx: 0 });
        }
        for ch in s.as_bytes() {
            let size = queue[*ch as usize - b'a' as usize].len();
            for _ in 0..size {
                let mut p = queue[*ch as usize - b'a' as usize].pop_front().unwrap();
                p.idx += 1;
                if p.curr.len() == p.idx as usize {
                    cnt += 1;
                    continue;
                }
                queue[p.curr.as_bytes()[p.idx as usize] as usize - b'a' as usize].push_back(p);
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0792() {
        assert_eq!(
            Solution::num_matching_subseq(
                String::from("abcde"),
                vec!["a", "bb", "acd", "ace"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            3
        );
        assert_eq!(
            Solution::num_matching_subseq(
                String::from("dsahjpjauf"),
                vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            2
        );
    }
}
