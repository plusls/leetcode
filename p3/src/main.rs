use std::io;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = i;
        let mut ret = 0_i32;
        let mut ascii_data = [0; 256];
        loop  {
            while j < s.len() {
                ascii_data[s[j] as usize] += 1;
                if !Solution::check_ascii_data(&ascii_data) {
                    break;
                }
                let l = (j - i + 1) as i32;
                if ret < l {
                    ret = l;
                }
                j += 1;
            }
            if j >= s.len() {
                break;
            }

            loop {
                ascii_data[s[i] as usize] -= 1;
                i += 1;
                j += 1;
                if j >= s.len() {
                    break;
                }
                ascii_data[s[j] as usize] += 1;
                if Solution::check_ascii_data(&ascii_data) {
                    ret += 1;
                    j += 1;
                    break;
                }
            }

            if j >= s.len() {
                break;
            }
        }
        ret
    }

    pub fn check_ascii_data(ascii_data: &[i32; 256]) -> bool {
        for n in ascii_data {
            if *n > 1 {
                return false;
            }
        }
        true
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
        println!("{}", Solution::length_of_longest_substring(input.to_string()));
    }
}
