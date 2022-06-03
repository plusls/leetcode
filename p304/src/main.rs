use std::io;

struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut ret = Self { matrix: vec![vec![0; matrix[0].len()]; matrix.len()] };
        for (i, row) in matrix.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                ret.matrix[i][j] = ret.sum_region(0, 0, i as i32, j as i32 - 1) +
                    ret.sum_region(0, j as i32, i as i32 - 1, j as i32) + column;
            }
        }
        ret
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if row1 == -1 || col1 == -1 || row2 == -1 || col2 == -1 {
            0
        } else if row1 == 0 && col1 == 0 {
            self.matrix[row2 as usize][col2 as usize]
        } else {
            self.matrix[row2 as usize][col2 as usize]
                - self.sum_region(0, 0, row1 - 1, col2)
                - self.sum_region(row1, 0, row2, col1 - 1)
        }
    }
}

fn main() {
    // 懒得写了，直接在 leetcode 上测的
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

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let k: i32 = input.parse().unwrap();
    }
}
