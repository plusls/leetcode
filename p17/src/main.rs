use std::io;

const DIGITS_DATA: [&str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let digits = digits.as_bytes();
        for ch in digits {
            let n = ch - b'0';
            let l = ret.len();
            let digits_data = DIGITS_DATA[n as usize];

            if l == 0 {
                for ch in digits_data.chars() {
                    ret.push(ch.to_string())
                }
            } else {
                for ch in digits_data[1..].chars() {
                    for i in 0..l {
                        let mut new_s = ret[i].clone();
                        new_s.push(ch);
                        ret.push(new_s);
                    }
                }
                let ch_0 = digits_data.chars().next().unwrap();
                for i in 0..l {
                    ret[i].push(ch_0);
                }
            }
        }
        ret
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();

        println!("{:?}", Solution::letter_combinations(input.to_string()));
    }
}
