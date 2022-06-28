impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::HashSet;
        let mut data = vec![0; 26];
        s.as_bytes().iter().for_each(|&c| {
            data[(c - b'a') as usize] += 1;
        });
        data.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = 0;
        let mut used = HashSet::new();
        data.iter().for_each(|&c| { if c != 0 { used.insert(c); } });
        for i in 0..data.len() - 1 {
            let n = data[i];
            if n == 0 {
                break;
            }
            if n == data[i + 1] {
                for j in 1_i32..27 {
                    let dst = n - j;
                    if dst == 0 {
                        ret += j;
                        break;
                    } else if !used.contains(&(n - j)) {
                        used.insert(n - j);
                        ret += j;
                        break;
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
        let input = input.trim();
        println!("{:?}", Solution::min_deletions(input.to_string()));
    }
}
