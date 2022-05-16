use std::io;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let ret_odd = Solution::longest_palindrome_internal(&s, true, 1);
        let mut l = (ret_odd.len() / 2) as i32;
        if l == 0 {
            l = 1;
        }

        let ret_not_odd = Solution::longest_palindrome_internal(&s, false, l);
        String::from_utf8(if ret_not_odd.len() > ret_odd.len() {
            ret_not_odd
        } else {
            ret_odd
        }).unwrap()
    }

    pub fn calc_l_r(i: i32, l: i32, odd: bool) -> (i32, i32) {
        if odd {
            (i - (l - 1), i + l)
        } else {
            (i - (l - 1), i + l + 1)
        }
    }

    pub fn longest_palindrome_internal(s: &[u8], odd: bool, min_l: i32) -> Vec<u8> {
        assert!(!s.is_empty());
        let mut i = 0_i32;
        let mut l = min_l;
        let (mut l_idx, mut r_idx) = Solution::calc_l_r(i, l, odd);
        if l_idx < 0 {
            i -= l_idx;
            // fuck leetcode
            let t = Solution::calc_l_r(i, l, odd);
            l_idx = t.0;
            r_idx = t.1;
        }
        if r_idx as usize > s.len() {
            return Vec::new();
        }
        let mut current_s = &s[l_idx as usize..r_idx as usize];
        let mut ret: &[u8] = &[];


        loop {
            while !Solution::check_palindrome(current_s) {
                i += 1;
                // fuck leetcode
                let t = Solution::calc_l_r(i, l, odd);
                l_idx = t.0;
                r_idx = t.1;
                if r_idx as usize > s.len() {
                    break;
                }
                current_s = &s[l_idx as usize..r_idx as usize];
            }
            if r_idx as usize > s.len() {
                break;
            }

            loop {
                ret = current_s;
                l += 1;
                // fuck leetcode
                let t = Solution::calc_l_r(i, l, odd);
                l_idx = t.0;
                r_idx = t.1;
                if r_idx as usize > s.len() {
                    break;
                }
                if l_idx < 0 {
                    i -= l_idx;
                    // fuck leetcode
                    let t = Solution::calc_l_r(i, l, odd);
                    l_idx = t.0;
                    r_idx = t.1;
                    if r_idx as usize > s.len() {
                        break;
                    }
                    // println!("{} {}", l_idx, r_idx);
                    current_s = &s[l_idx as usize..r_idx as usize];
                    break;
                }
                current_s = &s[l_idx as usize..r_idx as usize];
                if current_s[0] != current_s[current_s.len() - 1] {
                    break;
                }
            }

            if r_idx as usize > s.len() {
                break;
            }
        }
        Vec::from(ret)
    }

    pub fn check_palindrome(s: &[u8]) -> bool {
        // println!("check: {}", String::from_utf8(Vec::from(s)).unwrap());
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
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
        println!("{}", Solution::longest_palindrome(input.to_string()));
    }
}
