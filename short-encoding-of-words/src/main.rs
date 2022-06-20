impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|a, b| {
            a.as_bytes().iter().rev().cmp(b.as_bytes().iter().rev())
        });
        let mut ans = 0;
        for i in 0..words.len() - 1 {
            if !words[i + 1].ends_with(&words[i]) {
                ans += words[i].len() + 1;
            }
        }
        ans += words[words.len() - 1].len() + 1;
        // println!("{:?}", words);
        ans as i32
    }
}

struct Solution;


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let mut words: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            words.push(n.trim().replace('"', ""));
        }
        println!("{:?}", Solution::minimum_length_encoding(words));
    }
}
