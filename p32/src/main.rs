use std::io;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut char_stack: Vec<u8> = Vec::new();
        let mut len_stack: Vec<u32> = Vec::new();
        let mut ret = 0;
        for ch in s.as_bytes() {
            let ch = *ch;
            let top_ch = if !char_stack.is_empty() { char_stack[char_stack.len() - 1] } else { b'\0' };
            if ch == b'(' {
                char_stack.push(b'(');
                len_stack.push(1);
            } else if top_ch == b'(' {
                char_stack.pop();
                let mut n = 1_u32;
                loop {
                    let top_n = len_stack.pop().unwrap();
                    n += top_n;
                    if top_n == 1 {
                        break;
                    }
                }
                loop {
                    let top_n = if !len_stack.is_empty() { len_stack[len_stack.len() - 1] } else { 1 };
                    if top_n == 1 {
                        break;
                    }
                    n += top_n;
                    len_stack.pop();
                }
                if n > ret {
                    ret = n;
                }
                len_stack.push(n);
            } else {
                char_stack.push(b')');
                len_stack.push(1);
            }
        }
        ret as i32
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

        println!("{:?}", Solution::longest_valid_parentheses(input.to_string()));
    }
}
