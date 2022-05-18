use std::io;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // println!("{:?}", matrix);
        // println!("k:{}", k);
        let row_l = matrix.len();
        let column_l = matrix[0].len();
        let mut l = matrix[0][0];
        let mut r = matrix[row_l - 1][column_l - 1] + 1;
        let mut t = 0;
        loop {
            let mid = if (l + r) % 2 == 0 { (l + r) / 2} else { (l + r - 1) /2};
            let (n, n_ret, n_ret_num) = Solution::get_idx(&matrix, mid);
            // println!("l:{}, r:{}, mid:{}, n:{}, n_ret:{}, n_ret_num:{}", l, r, mid, n, n_ret, n_ret_num);
            if k > n - n_ret_num && k <= n {
                return n_ret;
            } else if k > n {
                assert_ne!(l, mid);
                l = mid + 1;
                assert_ne!(l, r);
            } else {
                r = mid;
                assert_ne!(l, r);
            }
            t += 1;
            if t == 30 {
                return 114514;
            }
        }
    }

    pub fn get_idx(matrix: &[Vec<i32>], n: i32) -> (i32, i32, i32) {
        // 返回 (<= n 的个数, 矩阵里 <= n 且最接近 n 的数 n_ret, n_ret 的个数)
        let row_l = matrix.len();
        let column_l = matrix[0].len();
        let mut i = row_l - 1;
        let mut j = 0;
        let mut ret = 0_i32;
        let mut n_ret = i32::MIN;
        let mut n_ret_num = 0;
        loop {
            // println!("i:{} j:{}", i, j);
            if matrix[i][j] <= n {
                while j + 1 < column_l && matrix[i][j + 1] <= n {
                    j += 1;
                }
                if n_ret <= matrix[i][j] {
                    if n_ret < matrix[i][j] {
                        n_ret = matrix[i][j];
                        n_ret_num = 0;
                    }
                    let mut k = j;
                    // println!("i:{} j:{} matrix[i][j]:{}", i, j, matrix[i][j]);
                    while matrix[i][k] == n_ret {
                        n_ret_num += 1;
                        if k == 0 {
                            break;
                        }
                        k -= 1;
                    }
                }
                ret += j as i32 + 1;
            }
            if i != 0 {
                i -= 1;
            } else {
                break;
            }
        }
        (ret, n_ret, n_ret_num)
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

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let k: i32 = input.parse().unwrap();

        println!("{}", Solution::kth_smallest(matrix, k));
    }
}
