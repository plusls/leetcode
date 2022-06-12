impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![vec![0; 4]; s.len()]; s.len()];
        for len in 0..s.len() {
            for i in 0..s.len() {
                let j = i + len;
                if j >= s.len() {
                    break;
                }
                let ch = s[i] - b'a';
                if i == j {
                    dp[i][j][ch as usize] = 1;
                } else {
                    for k in 0..4 {
                        if k == ch {
                            if s[j] - b'a' == ch {
                                let mut new_dp = 2;
                                if i + 1 != j {
                                    for l in 0..4 {
                                        new_dp += dp[i + 1][j - 1][l];
                                        new_dp %= 1000000007;
                                    }
                                }

                                dp[i][j][k as usize] = new_dp;
                            } else {
                                dp[i][j][k as usize] = dp[i][j - 1][k as usize];
                            }
                        } else if k == s[j] - b'a' {
                            dp[i][j][k as usize] = dp[i + 1][j][k as usize];
                        } else if i + 1 != j {
                            dp[i][j][k as usize] = dp[i + 1][j - 1][k as usize];
                        }
                    }
                }
            }
        }
        let mut new_dp = 0;
        for i in 0..4 {
            new_dp += dp[0][s.len() - 1][i];
            new_dp %= 1000000007;
        }
        new_dp
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
        println!("{:?}", Solution::count_palindromic_subsequences(input.trim().to_string()));
    }
}
