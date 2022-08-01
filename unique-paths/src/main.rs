impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        let mut ret = 1_i64;
        if n > m {
            std::mem::swap(&mut m, &mut n);
        }
        for i in 0..n as i64 - 1 {
            ret *= m as i64 + i;
            ret /= i + 1;
        }

        ret as i32
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
        let m: i32 = input.trim().parse().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();


        println!("{:?}", Solution::generate_the_string(m, n));
    }
}
