impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        use std::cmp::max;
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; n]; m];
        for (i, ch1) in word1.iter().enumerate() {
            for (j, ch2) in word2.iter().enumerate() {
                if ch1 == ch2 {
                    dp[i][j] = if i != 0 && j != 0 { dp[i - 1][j - 1] } else { 0 } + 1;
                } else {
                    dp[i][j] = max(if i != 0 { dp[i - 1][j] } else { 0 }, if j != 0 { dp[i][j - 1] } else { 0 });
                }
            }
            // dp[i][0] = i as i32;
        }
        m as i32 + n as i32 - dp[m - 1][n - 1] * 2
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
        let word1 = input.trim().to_string();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let word2 = input.trim().to_string();

        println!("{:?}", Solution::min_distance(word1, word2));
    }
}
