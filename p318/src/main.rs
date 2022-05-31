use std::io;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;
        use std::cmp::max;
        let mut new_words: HashMap<usize, usize> = HashMap::new();
        for word in words {
            let mut key: usize = 0;
            for ch in word.as_bytes() {
                let ch = ch - b'a';
                key |= 1 << ch;
            }
            match new_words.entry(key) {
                Entry::Occupied(mut entry) => {
                    entry.insert(max(*entry.get(), word.len()));
                }
                Entry::Vacant(entry) => {
                    entry.insert(word.len());
                }
            }
        }
        let mut ret = 0;
        for (k0, v0) in &new_words {
            for (k1, v1) in &new_words {
                if k0 & k1 == 0 && v0 * v1 > ret {
                    ret = v0 * v1;
                }
            }
        }
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
        let mut words: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            words.push(n.trim().replace('"', ""));
        }
        println!("{:?}", Solution::max_product(words));
    }
}
