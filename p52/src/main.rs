use std::io;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut ret = 0;
        let mut states: Vec<i32> = Vec::new();
        Self::solve_n_queens_dfs(n, &mut states, &mut ret);
        ret
    }
    pub fn solve_n_queens_dfs(n: i32, current_state: &mut Vec<i32>, ret: &mut i32) {
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
                *ret += 1;
            } else {
                Self::solve_n_queens_dfs(n, current_state, ret);
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

        println!("{:?}", Solution::total_n_queens(input.parse().unwrap()));
    }
}
