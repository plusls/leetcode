impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        words.sort_unstable_by_key(|w| w.len());
        let mut dp = vec![1_i32; words.len()];
        let mut map: HashMap<Vec<u8>, usize> = HashMap::new();
        for (i, w) in words.iter().enumerate() {
            map.insert(w.as_bytes().to_vec(), i);
        }
        let mut max_v = 1;
        for idx in 0..words.len() {
            let w = words[idx].as_bytes();
            for i in 0..w.len() {
                let mut w = w.to_vec();
                w.remove(i);
                if let Some(j) = map.get(&w) {
                    let new_dp_v = 1 + dp[*j];
                    if new_dp_v > dp[idx] {
                        if new_dp_v > max_v {
                            max_v = new_dp_v;
                        }
                        dp[idx] = new_dp_v;
                    }
                }
            }
        }
        max_v
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
        println!("{:?}", Solution::longest_str_chain(words));
    }
}
