use std::io;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let mut states: Vec<i32> = Vec::new();
        Self::solve_n_queens_dfs(n, &mut states, &mut results);
        results
    }
    pub fn solve_n_queens_dfs(n: i32, current_state: &mut Vec<i32>, results: &mut Vec<Vec<String>>) {
        let x = current_state.len() as i32;
        for y in 0..n {
            let mut next = false;
            for x0 in 0..x {
                let y0 = current_state[x0 as usize];
                if x0 == x || y0 == y || (x - x0).abs() == (y - y0).abs() {
                    next = true;
                    break;
                }
            }
            if next {
                continue;
            }
            current_state.push(y);
            if x == n - 1 {
                let mut new_results: Vec<String> = Vec::new();
                for pos in current_state.iter() {
                    let mut new_result = String::new();
                    for i in 0..n {
                        if i == *pos {
                            new_result.push('Q');
                        } else {
                            new_result.push('.');
                        }
                    }
                    new_results.push(new_result);
                }
                results.push(new_results);
            } else {
                Self::solve_n_queens_dfs(n, current_state, results);
            }
            current_state.pop();
        }
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

        println!("{:?}", Solution::solve_n_queens(input.parse().unwrap()));
    }
}
