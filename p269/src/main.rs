use std::io;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        use std::cmp::min;
        use std::collections::HashSet;
        use std::collections::VecDeque;

        if words.len() == 1 {
            return words[0].clone();
        }
        // println!("{:?}", words);
        let mut alpha_idx_map: Vec<usize> = vec![114514; 26];
        let mut idx_alpha_map: Vec<usize> = vec![114514; 26];

        for word in &words {
            for ch in word.as_bytes() {
                alpha_idx_map[(*ch - b'a') as usize] = 0;
            }
        }
        let mut count = 0_usize;
        for (i, alpha) in alpha_idx_map.iter_mut().enumerate() {
            if *alpha == 0 {
                *alpha = count;
                idx_alpha_map[count] = i;
                count += 1;
            }
        }
        let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); count];
        let mut in_degrees = vec![0; count];
        for i in 0..words.len() - 1 {
            let s1 = words[i].as_bytes();
            let s2 = words[i + 1].as_bytes();
            let len = min(s1.len(), s2.len());
            let mut breaked: bool = false;
            for j in 0..len {
                let s1_j = alpha_idx_map[(s1[j] - b'a') as usize];
                let s2_j = alpha_idx_map[(s2[j] - b'a') as usize];
                if s1_j != s2_j {
                    if g[s1_j].insert(s2_j) {
                        in_degrees[s2_j] += 1;
                    }
                    breaked = true;
                    break;
                }
            }
            if !breaked && s1.len() > s2.len() {
                return "".to_string();
            }
        }

        let mut deque: VecDeque<usize> = VecDeque::new();
        for (i, in_degree) in in_degrees.iter().enumerate() {
            if *in_degree == 0 {
                deque.push_back(i);
            }
        }
        let mut results: Vec<usize> = Vec::new();
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            results.push(node);
            for target_node in &g[node] {
                let target_node = *target_node;
                in_degrees[target_node] -= 1;
                if in_degrees[target_node] == 0 {
                    deque.push_back(target_node);
                }
            }
        }
        // println!("in_degrees:{:?}", in_degrees);
        if results.len() != count {
            "".to_string()
        } else {
            let mut ret = String::new();
            results.iter().for_each(|result| ret.push((idx_alpha_map[*result] as u8 + b'a') as char));
            ret
        }
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
        println!("{:?}", Solution::alien_order(words));
    }
}
