use std::collections::HashMap;

#[derive(Default)]
struct FreqStack {
    frq: HashMap<i32, i32>,
    data: Vec<Vec<i32>>,
    rest: Vec<Vec<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        // get freq of val
        let curr = self.frq.entry(val).or_insert(0);
        if let Some(data) = self.data.get_mut(*curr as usize) {
            data.push(val);
        } else {
            if let Some(mut x) = self.rest.pop() {
                x.push(val);
                self.data.push(x);
            } else {
                self.data.push(vec![val]);
            }
        }
        *curr += 1
    }

    fn pop(&mut self) -> i32 {
        if let Some(mut x) = {
            let s = self.data.pop();
            s
        } {
            if let Some(val) = x.pop() {
                *self.frq.get_mut(&val).unwrap_or(&mut 0) -= 1;
                if x.len() > 0 {
                    self.data.push(x)
                } else {
                    self.rest.push(x)
                }
                val
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n0895() {
        let mut obj = FreqStack::new();
        vec![5, 7, 5, 7, 4, 5].iter().for_each(|&n| obj.push(n));
        let mut got: Vec<i32> = Vec::new();
        for _ in 0..4 {
            got.push(obj.pop());
        }
        assert_eq!(got, vec![5, 7, 5, 4]);
    }
}
