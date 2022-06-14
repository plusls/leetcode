impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let (mut i, mut j) = (0_usize, 0_usize);
        let mut flag = true;
        let mut ret: Vec<i32> = Vec::new();
        loop {
            // println!("{} {}", i, j);
            ret.push(mat[i][j]);
            if i == m - 1 && j == n - 1 {
                return ret;
            }
            if flag {
                if i == 0 {
                    if j == n - 1 {
                        i += 1;
                    } else {
                        j += 1;
                    }
                    flag = false;
                } else if j == n - 1 {
                    i += 1;
                    flag = false;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else if j == 0 {
                if i == m - 1 {
                    j += 1;
                } else {
                    i += 1;
                }
                flag = true;
            } else if i == m - 1 {
                j += 1;
                flag = true;
            } else {
                i += 1;
                j -= 1;
            }
        }
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
        let input = input.trim();

        let mut mat: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            mat.push(Vec::new());
            let l = mat.len();
            for column in row.split(',') {
                mat[l - 1].push(column.trim().parse().unwrap());
            }
        }
        println!("{:?}", Solution::find_diagonal_order(mat));
    }
}
