use std::io;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        let mut ret = 0_i32;
        let neg = s[0] == b'-';
        for (i, ch) in s.iter().enumerate() {
            if *ch >= b'0' && *ch <= b'9' {
                ret = ret.saturating_mul(10);

                ret = if neg {
                    ret.saturating_sub((ch - b'0') as i32)
                } else {
                    ret.saturating_add((ch - b'0') as i32)
                };
                // println!("ret:{}", ret);
                if ret == i32::MAX || ret == i32::MIN {
                    break;
                }
            } else if i == 0 {
                if *ch == b'-' || *ch == b'+' {
                    continue;
                } else {
                    return 0;
                }
            } else {
                break;
            }
        }

        ret
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        println!("{}", Solution::my_atoi(input));
    }
}
