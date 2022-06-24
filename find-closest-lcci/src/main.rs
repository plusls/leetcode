impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        // println!("{:?} {:?} {:?}", words, word1, word2);
        let mut p1 = -1;
        let mut p2 = -1;
        let mut ret = -1;
        for (i, word) in words.iter().enumerate() {
            if *word == word1 {
                p1 = i as i32;
                if p2 != -1 {
                    let new_ret = if p1 > p2 {
                        p1 - p2
                    } else {
                        p2 - p1
                    };
                    if ret == -1 || ret > new_ret {
                        ret = new_ret;
                    }
                }
            }
            if *word == word2 {
                p2 = i as i32;
                if p1 != -1 {
                    let new_ret = if p1 > p2 {
                        p1 - p2
                    } else {
                        p2 - p1
                    };
                    if ret == -1 || ret > new_ret {
                        ret = new_ret;
                    }
                }
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
        let mut words: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            words.push(n.trim().replace('"', ""));
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut word_v: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            word_v.push(n.trim().replace('"', ""));
        }
        println!("{:?}", Solution::find_closest(words, word_v[0].clone(), word_v[1].clone()));
    }
}
