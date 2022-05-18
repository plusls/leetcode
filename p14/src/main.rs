use std::io;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut run = true;
        let strs = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let mut i = 0;
        while run {
            if i >= strs[0].len() {
                break;
            }
            let ch = strs[0][i];
            for s in &strs {
                if i >= s.len() || s[i] != ch {
                    run = false;
                    break;
                }
            }
            if run {
                i += 1;
            }
        }
        std::str::from_utf8(&strs[0][0..i]).unwrap().to_string()
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut strs: Vec<String> = Vec::new();
        for n in input.split(',') {
            strs.push(n.to_string());
        }
        println!("{}", Solution::longest_common_prefix(strs));
    }
}
