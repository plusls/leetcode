impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // println!("s:{:?}, words:{:?}", s, words);
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;
        let s = s.as_bytes();
        // idx, 出现的次数
        let mut map: HashMap<Vec<u8>, (usize, usize)> = HashMap::new();
        let mut idx = 0_usize;
        let word_len = words[0].len();
        let words_len = words.len();

        let mut required_idx_vec: Vec<usize> = Vec::new();

        for word in words {
            match map.entry(word.as_bytes().to_vec()) {
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().1 += 1;
                    required_idx_vec[occupied.get_mut().0] += 1;
                }
                Entry::Vacant(vacant) => {
                    vacant.insert((idx, 1));
                    required_idx_vec.push(1);
                    idx += 1;
                }
            }
        }

        // println!("required_idx_vec:{:?}", required_idx_vec);
        let mut ret = Vec::new();

        for i in 0..word_len {
            let mut s_vec: Vec<(i32, i32)> = Vec::new();
            let mut current_idx = i;
            while current_idx + word_len <= s.len() {
                if let Some(v) = map.get(&s[current_idx..current_idx + word_len]) {
                    s_vec.push((current_idx as i32, v.0 as i32));
                } else {
                    s_vec.push((-1, -1));
                }
                current_idx += word_len;
            }
            // println!("s_vec:{:?}", s_vec);
            let mut current_l = 0;
            let mut current_r;
            while current_l < s_vec.len() {
                while current_l < s_vec.len() && s_vec[current_l].0 == -1 {
                    current_l += 1;
                }
                if current_l == s_vec.len() {
                    break;
                }
                current_r = current_l;
                while current_r + 1 < s_vec.len() && s_vec[current_r + 1].0 != -1 {
                    current_r += 1;
                }
                // println!("current_l:{:?}, current_r:{:?}", current_l, current_r);

                if current_r - current_l + 1 >= words_len {
                    let mut current_idx_vec = vec![0; required_idx_vec.len()];

                    for j in 0..words_len {
                        current_idx_vec[s_vec[current_l + j].1 as usize] += 1;
                    }

                    let mut cache_flag = false;
                    for j in 0..current_r - current_l + 2 - words_len {
                        if cache_flag {
                            ret.push(s_vec[current_l + j].0);
                        } else if current_idx_vec == required_idx_vec {
                            ret.push(s_vec[current_l + j].0);
                            cache_flag = true;
                        } else {
                            cache_flag = false;
                        }
                        // println!("ret:{:?} j:{} {}", ret, j, current_r - current_l + 1 - words_len);
                        if j == current_r - current_l + 1 - words_len {
                            break;
                        }
                        let a = s_vec[current_l + j].1;
                        let b = s_vec[current_l + words_len + j].1;

                        current_idx_vec[a as usize] -= 1;
                        current_idx_vec[b as usize] += 1;

                        if cache_flag {
                            cache_flag = a == b;
                        }
                    }
                }
                current_l = current_r + 1;
            }
        }


        ret
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
        let s = input.trim().to_string();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut words: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            words.push(n.trim().replace('"', ""));
        }
        println!("{:?}", Solution::find_substring(s, words));
    }
}
