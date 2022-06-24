impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<(i32, i32, i32)> = Vec::new();
        for (i, cost) in costs.iter().enumerate() {
            if i == 0 {
                dp.push((cost[0], cost[1], cost[2]));
            } else {
                let a = cost[0] + dp[i - 1].1.min(dp[i - 1].2);
                let b = cost[1] + dp[i - 1].0.min(dp[i - 1].2);
                let c = cost[2] + dp[i - 1].0.min(dp[i - 1].1);
                dp.push((a, b, c));
            }
        }
        let (a, b, c) = dp[costs.len() - 1];
        a.min(b).min(c)
    }
}

struct Solution();


fn main() {
    use std::io;

    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim().replace(' ', "");
        let mut costs: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            costs.push(Vec::new());
            let l = costs.len();
            for column in row.split(',') {
                costs[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::min_cost(costs));
    }
}
