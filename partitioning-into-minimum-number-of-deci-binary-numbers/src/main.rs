impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut ret = 0;
        for &ch in n.as_bytes() {
            ret = ret.max(ch - b'0');
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

        println!("{}", Solution::min_partitions(input.trim().to_string()));
    }
}
