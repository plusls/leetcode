impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut current_n = n;
        let mut i = current_n % 10;
        let mut i_idx = 0;
        loop {
            let prev_i = i;
            i = current_n % 10;
            if prev_i > i {
                break;
            }
            current_n /= 10;
            if current_n == 0 {
                return -1;
            }
            i_idx += 1;
        }
        let mut current_n = n;
        let mut j = current_n % 10;
        let mut j_idx = 0;
        loop {
            if j > i {
                break;
            }
            current_n /= 10;
            j = current_n % 10;
            j_idx += 1;
        }
        let ret = match (n / 10_i32.pow(i_idx + 1) * 10_i32.pow(i_idx + 1)).checked_add(
            match j.checked_mul(10_i32.pow(i_idx)) {
                None => { return -1; }
                Some(r) => { r }
            }) {
            None => { return -1; }
            Some(r) => { r }
        };
        let mut x = n % 10_i32.pow(i_idx) - (j - i) * 10_i32.pow(j_idx);
        // println!("ret: {}, i:{}, i_idx:{}, j:{}, j_idx:{}, x:{}", ret, i, i_idx, j, j_idx, x);

        let mut rev_x = 0;
        while x != 0 {
            rev_x *= 10;
            rev_x += x % 10;
            x /= 10;
        }

        match ret.checked_add(rev_x) {
            None => { -1 }
            Some(r) => { r }
        }
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
        println!("{:?}", Solution::next_greater_element(input.trim().parse().unwrap()));
    }
}
