use std::io;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut s1 = Vec::new();
        for ch in s.chars() {
            s1.push('#');
            s1.push(ch);
        }
        s1.push('#');
        let s = s1;

        let s_l = s.len();
        let mut p = vec![0; s_l];
        let mut i = 0;
        let mut c = i;
        let mut r = i + p[c];
        while i < s_l {
            let mut j = if i > c && i <= r {
                let p_i = p[i];
                if p_i + i > r {
                    r + 1
                } else {
                    p_i + 1
                }
            } else { 1 };
            loop {
                if i < j {
                    break;
                }
                let l_idx = i - j;
                let r_idx = i + j;
                if i + j >= s_l {
                    break;
                }
                if s[l_idx] != s[r_idx] {
                    break;
                }
                j += 1;
            }
            p[i] = j - 1;
            if i + p[i] > r {
                c = i;
                r = i + p[c];
            }
            i += 1;
        }
        let mut ret = 0_i32;
        for p_i in p {
            let p_i = p_i as i32;
            ret += (p_i + 1) / 2;
        }
        ret
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

        println!("{}", Solution::count_substrings(input.to_string()));
    }
}
