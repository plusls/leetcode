use std::io;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        // println!("s:{}, k:{}", s, k);
        let n = 1 << k;
        if s.len() as i32 - k + 1 < n {
            return false;
        }
        let mut data = vec![false; n as usize];
        for i in 0..(s.len() - k as usize + 1) {
            data[Self::get_usize(&s, i, k as usize)] = true;
        }
        for b in data {
            if !b {
                return false;
            }
        }
        true
    }
    pub fn get_usize(s: &str, start_idx: usize, len: usize) -> usize {
        // println!("s:{:?}, start_idx:{}, len:{}", &s.as_bytes()[start_idx..start_idx + len],start_idx, len);
        let mut ret: usize = 0;
        for ch in &s.as_bytes()[start_idx..start_idx + len] {
            ret <<= 1;
            if *ch == b'1' {
                ret |= 1;
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
        let s = input.trim();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let k: i32 = input.trim().parse().unwrap();
        println!("{:?}", Solution::has_all_codes(s.to_string(), k));
    }
}
