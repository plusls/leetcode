use std::io;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // println!("x:{}", x);
        let mut x = x;
        let neg = x < 0;
        if neg {
            x = -x;
        }
        let mut ret = 0_i32;
        while x != 0 {
            ret = match ret.checked_mul(10) {
                None => { return 0; }
                Some(r) => { r }
            };
            ret = match ret.checked_add(x % 10) {
                None => { return 0; }
                Some(r) => { r }
            };

            x /= 10;
            // println!("ret:{}", ret);
        }
        if neg {
            ret = -ret;
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
        println!("{}", Solution::reverse(input.parse().unwrap()));
    }
}
