use std::io;

const ROMAN_DATA: [(u8, i32); 7] = [(b'I', 1), (b'V', 5),
    (b'X', 10), (b'L', 50),
    (b'C', 100), (b'D', 500),
    (b'M', 1000)
];

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut ret = 0_i32;
        let mut prev_n = 0;
        while i < s.len() {
            for (ch, n) in ROMAN_DATA {
                if s[i] == ch {
                    ret += n;
                    if prev_n != 0 && prev_n < n {
                        ret -= prev_n * 2;
                    }
                    i += 1;
                    prev_n = n;
                    break;
                }
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
        let input = input.trim();
        println!("{}", Solution::roman_to_int(input.to_string()));
    }
}
