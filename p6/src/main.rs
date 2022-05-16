use std::io;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1{
            return s
        }
        let s = s.as_bytes();
        let mut ret = Vec::new();
        let w = 2 * num_rows - 2;

        for i in 0..num_rows {
            let mut j = 0;
            let idx = i as usize;
            if idx >= s.len() {
                break;
            }
            // println!("i:{} j:{} idx:{}", i, j, idx);
            ret.push(s[idx]);
            loop {
                let k = (num_rows - i) * 2 - 2;
                let idx0 = (i + j * w + k) as usize;
                if k != 0 {
                    if idx0 >= s.len() {
                        break;
                    }
                    // println!("i:{} j:{} idx0:{}, k:{}", i, j, idx0, k);
                    ret.push(s[idx0]);
                }
                // 2 * num_rows - 2 - k
                let l = i * 2;
                let idx1 = (i + j * w + k + l) as usize;

                if l != 0 {
                    if idx1 >= s.len() {
                        break;
                    }
                    // println!("i:{} j:{} idx1:{} l:{}", i, j, idx1, l);
                    ret.push(s[idx1]);
                }
                j += 1;
            }
        }
        String::from_utf8(ret).unwrap()
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let s = input.trim();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{}", Solution::convert(s.to_string(), input.parse().unwrap()));
    }
}
