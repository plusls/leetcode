use std::io;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut ret = 0;
        while n != 0 {
            if n & 1 == 1 {
                ret += 1;
            }
            n >>= 1;
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

        println!("{}", Solution::hammingWeight(input.parse().unwrap()));
    }
}
