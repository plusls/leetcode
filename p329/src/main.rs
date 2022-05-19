use std::cmp::max;
use std::io;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        // 表示以 i j 为起点最多还能走几步
        let mut max_steps = vec![vec![0; n]; m];
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                let max_step = max_steps[i][j];
                let v = matrix[i][j];
                if max_step == 0 {
                    // println!("new");
                    ret = max(ret, Self::get_max_step(&mut max_steps, &matrix, i as i32, j as i32, i32::MIN));
                }
            }
        }
        ret
    }
    pub fn get_max_step(max_steps: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>, x: i32, y: i32, prev_v: i32) -> i32 {
        // println!("dp_data:{:?} matrix:{:?} x:{} y:{} prev_v:{} prev_dp_v:{}", dp_data, matrix, x, y, prev_v, prev_dp_v);
        let m = matrix.len();
        let n = matrix[0].len();
        if x < 0 || x as usize >= m || y < 0 || y as usize >= n {
            return 0;
        }
        let x = x as usize;
        let y = y as usize;
        let v = matrix[x][y];
        if v <= prev_v {
            return 0;
        }
        let mut max_step = max_steps[x][y];
        // println!("fuck");

        if max_step != 0 {
            return max_step;
        }
        max_step = 1;
        let mut ret = max_step;
        ret = max(ret, max_step + Self::get_max_step(max_steps, &matrix, x as i32 - 1, y as i32, v));
        ret = max(ret, max_step + Self::get_max_step(max_steps, &matrix, x as i32 + 1, y as i32, v));
        ret = max(ret, max_step + Self::get_max_step(max_steps, &matrix, x as i32, y as i32 - 1, v));
        ret = max(ret, max_step + Self::get_max_step(max_steps, &matrix, x as i32, y as i32 + 1, v));
        max_steps[x][y] = ret;
        ret
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
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            matrix.push(Vec::new());
            let l = matrix.len();
            for column in row.split(',') {
                matrix[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::longest_increasing_path(matrix));
    }
}
