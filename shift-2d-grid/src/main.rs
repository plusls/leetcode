impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut ret = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let pos = (i * grid[0].len() + j + k as usize) % (grid[0].len() * grid.len());
                ret[pos / grid[0].len()][pos % grid[0].len()] = grid[i][j];

                // println!("pos: {} ({},{}) <- ({},{})", pos, i, j, pos / grid[0].len(), pos % grid[0].len());
            }
        }
        ret
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
        let mut grid: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            grid.push(Vec::new());
            let l = grid.len();
            for column in row.split(',') {
                grid[l - 1].push(column.parse().unwrap());
            }
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", Solution::shift_grid(grid, input.trim().parse().unwrap()));
    }
}
