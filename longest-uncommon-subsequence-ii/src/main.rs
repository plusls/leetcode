impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        use std::cmp::Reverse;
        use std::str;
        strs.sort_unstable_by_key(|b| Reverse(b.len()));
        println!("strs: {:?}", strs);
        for i in 0..strs.len() {
            let mut flag = true;
            let s1 = strs[i].as_bytes();
            for (j, w) in strs.iter().enumerate() {
                let s2 = w.as_bytes();
                if i == j {
                    continue;
                }
                if s1.len() > s2.len() {
                    continue;
                }
                let mut s1_idx = 0_usize;
                let mut s2_idx = 0_usize;
                while s1_idx < s1.len() && s2_idx < s2.len() {
                    if s1.len() - s1_idx > s2.len() - s2_idx {
                        break;
                    }
                    if s1[s1_idx] == s2[s2_idx] {
                        s1_idx += 1;
                        s2_idx += 1;
                    } else {
                        s2_idx += 1;
                    }
                }
                if s1_idx == s1.len() {
                    flag = false;
                }
                println!("s1: {:?}, s2: {:?}, flag: {}, s1_idx:{}, s2_idx:{}", str::from_utf8(s1).unwrap(), str::from_utf8(s2).unwrap(), flag, s1_idx, s2_idx);
                if !flag {
                    break;
                }
            }
            if flag {
                return s1.len() as i32;
            }
        }
        -1
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
        let mut words: Vec<String> = Vec::new();
        for n in input.split("\",\"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            words.push(n);
        }
        println!("{:?}", Solution::find_lu_slength(words));
    }
}
