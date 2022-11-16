use super::Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let split = sentence.split(" ").collect::<Vec<&str>>();
        let mut sb_vec: Vec<String> = Vec::with_capacity(split.len());
        for (idx, s) in split.iter().enumerate() {
            let mut sb: String;
            match s.chars().nth(0).unwrap().to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    sb = String::from(s.clone());
                }
                _ => {
                    sb = String::from(s.clone());
                    let head = sb.remove(0);
                    sb.push(head);
                }
            }
            sb.push_str("ma");
            sb.push_str("a".repeat(idx + 1).as_str());
            sb_vec.push(sb);
        }
        sb_vec.join(" ")
    }
}
