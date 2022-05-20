use std::cmp;
use std::io;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if m == 1 && n == 1 && obstacle_grid[0][0] == 1 {
            return 0;
        }
        let mut results = vec![vec![0; n]; m];
        results[0][0] = 1;
        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                let mut u = 1;
                let mut l = 1;
                if i != 0 {
                    u = obstacle_grid[i - 1][j];
                }
                if j != 0 {
                    l = obstacle_grid[i][j - 1];
                }
                if u != 1 {
                    results[i][j] += results[i - 1][j];
                }
                if l != 1 {
                    results[i][j] += results[i][j - 1];
                }
            }
        }
        results[m - 1][n - 1] as i32
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut obstacle_grid: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            obstacle_grid.push(Vec::new());
            let l = obstacle_grid.len();
            for column in row.split(',') {
                obstacle_grid[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{}", Solution::unique_paths_with_obstacles(obstacle_grid));
    }
}
