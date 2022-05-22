use std::io;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // println!("coins: {:?}, amount: {}", coins, amount);
        let mut dp: Vec<i32> = vec![-1; amount as usize + 1];
        dp[0] = 0;
        for i in 0..amount as usize + 1 {
            let mut dp_i = dp[i];
            for coin in coins.iter() {
                let prev_dp_idx = i as i32 - coin;
                if prev_dp_idx >= 0 {
                    let prev_dp = dp[prev_dp_idx as usize];
                    // println!("i: {}, prev_dp_idx: {}, prev_dp: {}, dp_i: {}", i, prev_dp_idx, prev_dp, dp_i);
                    if prev_dp != -1 && (dp_i == -1 || dp_i > prev_dp) {
                        dp_i = prev_dp + 1;
                    }
                }
            }
            dp[i] = dp_i;
        }
        // println!("dp: {:?}", dp);
        dp[amount as usize]
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
        let mut coins: Vec<i32> = Vec::new();
        for n in input.split(',') {
            coins.push(n.replace('[', "").replace(']', "").trim().parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{}", Solution::coin_change(coins, input.parse().unwrap()));
    }
}
