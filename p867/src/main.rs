use std::io;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ret = vec![vec![0; m]; n];
        for (i, row) in matrix.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                ret[j][i] = *column;
            }
        }
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
                matrix[l - 1].push(column.trim().parse().unwrap());
            }
        }
        println!("{:?}", Solution::transpose(matrix));
    }
}
