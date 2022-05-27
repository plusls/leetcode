impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ret = String::new();
        let mut l_idx = 0;
        let mut l_count = 0_u32;
        let mut r_count = 0_u32;
        let s: Vec<char> = s.chars().collect();
        for (i, ch) in s.iter().enumerate() {
            if *ch == '(' {
                l_count += 1;
            } else {
                r_count += 1;
                if l_count == r_count {
                    for ch in &s[l_idx + 1..i] {
                        ret.push(*ch);
                    }
                    l_idx = i + 1;
                    l_count = 0;
                    r_count = 0;
                }
            }
        }
        ret
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
        let input = input.trim();

        println!("{:?}", Solution::remove_outer_parentheses(input.to_string()));
    }
}