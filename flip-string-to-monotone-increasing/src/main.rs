impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        use std::cmp::min;
        let mut dp0 = 0;
        let mut dp1 = 0;
        for ch in s.chars() {
            match ch {
                '0' => {
                    dp1 = min(dp0, dp1) + 1;
                }
                '1' => {
                    dp1 = min(dp0, dp1);
                    dp0 += 1;
                }
                _ => {
                    panic!("wtf");
                }
            }
            // println!("dp0: {}, dp1: {}", dp0, dp1);
        }
        min(dp0, dp1)
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

        println!("{:?}", Solution::min_flips_mono_incr(input.to_string()));
    }
}
