impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return 2;
            }
        }
        1
    }
}

struct Solution();


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();

        println!("{:?}", Solution::remove_palindrome_sub(input.to_string()));
    }
}
