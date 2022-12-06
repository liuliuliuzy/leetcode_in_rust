use std::collections::HashSet;

use super::Solution;

// impl Solution {
//     pub fn num_different_integers(word: String) -> i32 {
//         let mut seen = HashSet::new();
//         let mut i: usize = 0;
//         let mut num: Vec<char> = vec![];
//         while i < word.len() {
//             if word.chars().nth(i).unwrap().is_ascii_digit() {
//                 num.clear();

//                 // 去除前导0
//                 while i < word.len()
//                     && word.chars().nth(i).unwrap().is_ascii_digit()
//                     && word.chars().nth(i).unwrap() == '0'
//                 {
//                     i += 1;
//                 }

//                 while i < word.len() && word.chars().nth(i).unwrap().is_ascii_digit() {
//                     num.push(word.chars().nth(i).unwrap());
//                     i += 1;
//                 }

//                 if num.len() == 0 {
//                     seen.insert(String::from("0"));
//                 } else {
//                     seen.insert(num.iter().collect::<String>());
//                 }
//             }
//             i += 1;
//         }
//         seen.len() as _
//     }
// }

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let v: Vec<&str> = word.split(char::is_alphabetic).collect();
        if v.len() == 1 {
            return 1;
        }
        let mut hset = HashSet::new();
        for i in v {
            if i.len() > 0 {
                let m = i.find(|c: char| c == '0');
                let n = i.find(|c: char| c != '0');
                if m.is_none() || (m.is_some() && n.is_some() && m.unwrap() > n.unwrap()) {
                    hset.insert(i);
                } else {
                    let t = i.trim_start_matches("0");
                    hset.insert(t);
                }
            }
        }
        hset.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1805() {
        assert_eq!(
            Solution::num_different_integers(String::from("a123bc34d8ef34")),
            3
        );
        assert_eq!(
            Solution::num_different_integers(String::from("leet1234code234")),
            2
        );
        assert_eq!(
            Solution::num_different_integers(String::from("a1b01c001")),
            1
        );

        assert_eq!(
            Solution::num_different_integers(String::from("0faer000frar010argrg00010gar11")),
            3
        );
    }
}
