impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        // println!("words: {:?}, pattern: {:?}", words, pattern);
        let mut ret: Vec<String> = Vec::new();
        let mut new_pattern = Vec::new();
        for ch in pattern.as_bytes() {
            new_pattern.push(ch - b'a');
        }
        for word in words {
            let mut w_p_maps = [u8::MAX; 26];
            let mut p_w_maps = [u8::MAX; 26];

            if word.len() != new_pattern.len() {
                continue;
            }
            // println!("word: {}", word);
            for (i, ch) in word.as_bytes().iter().enumerate() {
                let w_ch = ch - b'a';
                let p_ch = new_pattern[i];
                match w_p_maps[w_ch as usize] {
                    u8::MAX => {
                        if p_w_maps[p_ch as usize] != u8::MAX {
                            break;
                        }
                        w_p_maps[w_ch as usize] = p_ch;
                        p_w_maps[p_ch as usize] = w_ch;
                    }
                    p_ch1 => {
                        if p_ch1 != p_ch {
                            break;
                        }
                    }
                }
                if i == word.len() - 1 {
                    ret.push(word.clone());
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
        let input = input.trim();
        let mut words: Vec<String> = Vec::new();
        for n in input.split("\",\"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            words.push(n);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::find_and_replace_pattern(words, input.replace('"', "").trim().to_string()));
    }
}
