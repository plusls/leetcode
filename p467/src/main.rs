use std::io;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let p = p.as_bytes();
        let mut results = [0_usize; 26];
        let mut l = 0;
        let mut r = l;
        while r + 1 < p.len() {
            if p[r] + 1 != p[r + 1] && !(p[r] == b'z' && p[r + 1] == b'a') {
                let idx = (p[l] - b'a') as usize;
                if results[idx] < r - l + 1 {
                    results[idx] = r - l + 1;
                }
                l = r + 1;
            }
            r += 1;
        }
        let idx = (p[l] - b'a') as usize;
        if results[idx] < r - l + 1 {
            results[idx] = r - l + 1;
        }

        let mut current_max_n = 0;
        for _ in 0..2 {
            for n in results.iter_mut() {
                if *n > current_max_n {
                    current_max_n = *n;
                }
                if *n < current_max_n {
                    *n = current_max_n;
                }
                if current_max_n != 0 {
                    current_max_n -= 1;
                }
            }
        }


        let mut ret = 0;
        for n in results {
            ret += n;
        }

        // println!("results: {:?}", results);

        ret as i32
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        println!("{}", Solution::find_substring_in_wrapround_string(input.to_string()));
    }
}
