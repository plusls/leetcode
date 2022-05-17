use std::io;
use std::cmp;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order = order.as_bytes();
        let mut order_data = [0_usize; 256];
        for (i, ch) in order.iter().enumerate() {
            order_data[*ch as usize] = i;
        }
        for i in 0..words.len() - 1 {
            let w1 = words[i].as_bytes();
            let w2 = words[i + 1].as_bytes();
            let min_wl =  cmp::min(w1.len(), w2.len());
            for j in 0..min_wl {
                match order_data[w1[j] as usize].cmp(&order_data[w2[j] as usize]) {
                    cmp::Ordering::Less => {
                        break;
                    }
                    cmp::Ordering::Equal => {
                        if j == min_wl - 1 && w1.len() > w2.len() {
                            return false;
                        }
                        continue;
                    }
                    cmp::Ordering::Greater => {
                        return false;
                    }
                }
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
        let mut words = Vec::new();
        for n in input.split_whitespace() {
            words.push(n.to_string());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let order = input.trim().to_string();
        println!("{}", Solution::is_alien_sorted(words, order));
    }
}
