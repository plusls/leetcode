impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        for c in s.chars() {
            let count = map1.entry(c).or_insert(0);
            *count += 1;
        }
        for c in t.chars() {
            let count = map2.entry(c).or_insert(0);
            *count += 1;
        }
        map1 == map2
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

        println!("{:?}", Solution::is_anagram(s, input.trim().to_string()));
    }
}
