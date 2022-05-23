use std::io;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        use std::cmp::max;
        let m = m as usize;
        let n = n as usize;
        // println!("strs:{:?}, m:{}, n:{}", strs, m, n);
        let mut dp = vec![0; (m + 1) * (n + 1)];
        for (i, s) in strs.iter().enumerate() {
            for j in (0..(m + 1) * (n + 1)).rev() {
                let current_m = j / (n + 1);
                let current_n = j % (n + 1);
                let mut s_m = 0;
                let mut s_n = 0;
                s.chars().for_each(|ch| if ch == '0' {
                    s_m += 1;
                } else {
                    // assert_eq!(ch, '1');
                    s_n += 1;
                });
                if s_m > current_m || s_n > current_n {
                    dp[j] = if i >= 1 { dp[j] } else { 0 };
                    continue;
                }
                if i == 0 {
                    dp[j] = 1;
                } else {
                    dp[j] = max(dp[j - s_m * (n + 1) - s_n] + 1, dp[j]);
                    // println!("i:{} j:{} dp[i][j]: {} current_m:{} current_n:{} wtf:{}", i, j, dp[i - 1][j], current_m, current_n,
                    //          dp[i - 1][j - s_m * (n+1) - s_n] + 1);
                }
                if i == strs.len() - 1 {
                    break;
                }
            }
        }
        // println!("dp: {:?}", dp);
        dp[(m + 1) * (n + 1) - 1]
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut strs: Vec<String> = Vec::new();
        for n in input.split("\", \"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            strs.push(n);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for n in input.split_whitespace() {
            nums.push(n.trim().parse().unwrap());
        }
        println!("{}", Solution::find_max_form(strs, nums[0], nums[1]));
    }
}
