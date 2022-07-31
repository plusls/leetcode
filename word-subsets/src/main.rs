use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(mut words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut words1_map = Vec::new();
        for word in words1.iter() {
            let mut map = HashMap::new();
            for ch in word.chars() {
                match map.entry(ch) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(1);
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.insert(entry.get() + 1);
                    }
                }
            }
            words1_map.push(map);
        }
        let mut max_map = HashMap::new();
        for word in words2 {
            let mut map = HashMap::new();
            for ch in word.chars() {
                match map.entry(ch) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(1);
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.insert(entry.get() + 1);
                    }
                }
            }
            for (k, v) in map {
                match max_map.entry(k) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(v);
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        if v > *entry.get() {
                            entry.insert(v);
                        }
                    }
                }
            }
        }
        let mut ret = Vec::new();
        for (i, map) in words1_map.iter().enumerate() {
            if Self::check(map, &max_map) {
                ret.push(std::mem::replace(&mut words1[i], String::new()));
            }
        }
        ret
    }
    fn check(map: &HashMap<char, i32>, max_map: &HashMap<char, i32>) -> bool {
        for (k, v) in max_map.iter() {
            match map.get(k) {
                None => {
                    return false;
                }
                Some(map_v) => {
                    if map_v < v {
                        return false;
                    }
                }
            }
        }
        true
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
        let mut words1: Vec<String> = Vec::new();
        for n in input.split("\",\"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            words1.push(n);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut words2: Vec<String> = Vec::new();
        for n in input.split("\",\"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            words2.push(n);
        }
        println!("{:?}", Solution::word_subsets(words1, words2));
    }
}
