use std::io;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        for ch in s.as_bytes() {
            let ch = *ch;
            let top_ch = if !stack.is_empty() { stack[stack.len() - 1] } else { b'\0' };
            if ch == b'(' || ch == b'[' || ch == b'{' {
                stack.push(ch);
            } else if (ch == b')' && top_ch == b'(') || (ch == b']' && top_ch == b'[') || (ch == b'}' && top_ch == b'{') {
                stack.pop();
            } else {
                return false;
            }
        }
        stack.is_empty()
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

        println!("{:?}", Solution::is_valid(input.to_string()));
    }
}
