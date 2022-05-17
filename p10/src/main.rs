use std::io;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let s_l = s.len();
        let p_l = p.len();
        // data[p_l+1][s_l+1]
        // data[i][j] 表示 p[0..i] 匹配 s[0..j]
        let mut data: Vec<Vec<i8>> = vec![vec![-1; s_l + 1]; p_l + 1];

        for i in 0..p_l + 1 {
            for j in 0..s_l + 1 {
                data[i][j] = if i == 0 {
                    if j == 0 {
                        1
                    } else {
                        0
                    }
                } else if j == 0 {
                    if i % 2 == 0 && p[i - 1] == b'*' {
                        data[i - 2][0]
                    } else {
                        0
                    }
                } else if p[i - 1] != b'*' {
                    if data[i - 1][j - 1] == 1 && (p[i - 1] == s[j - 1] || p[i - 1] == b'.') {
                        1
                    } else {
                        0
                    }
                } else {
                    // 输入保证 p[0] != '*', 所以 i >= 2
                    if p[i - 2] == s[j - 1] || p[i - 2] == b'.' {
                        if data[i][j - 1] == 1 || data[i - 2][j] == 1 {
                            // 吃掉当前字符或者忽略掉 .*
                            1
                        } else {
                            0
                        }
                    } else {
                        // 没法吃掉当前字符，只能忽略掉 .*
                        data[i - 2][j]
                    }
                };
            }
        }
        assert_ne!(data[p_l][s_l], -1);
        //println!("data: {:?}", data);
        data[p_l][s_l] == 1
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let s = input.trim().to_string();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let p = input.trim().to_string();
        println!("{}", Solution::is_match(s, p));
    }
}
