use std::io;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        // println!("m:{} n:{} k:{}", m, n, k);
        let mut l = 1;
        let mut r = m * n + 1;
        loop {
            let mid = (l + r) / 2;
            let (n, n_ret, n_ret_num) = Solution::get_idx(m, n, mid);
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
        }
    }

    pub fn get_idx(m: i32, n: i32, num: i32) -> (i32, i32, i32) {
        // 返回 (<= n 的个数, 矩阵里 <= n 且最接近 n 的数 n_ret, n_ret 的个数)
        let mut i = m;
        let mut j = 1;
        let mut ret = 0_i32;
        let mut n_ret = i32::MIN;
        let mut n_ret_num = 0;
        loop {
            // println!("i:{} j:{}", i, j);
            if i * j <= num {
                while j < n && i * (j + 1) <= num {
                    j += 1;
                }
                if n_ret < i * j {
                    n_ret = i * j;
                    n_ret_num = 0;
                }
                let mut k = j;
                // println!("i:{} j:{} matrix[i][j]:{}", i, j, matrix[i][j]);
                while i * k == n_ret {
                    n_ret_num += 1;
                    if k == 1 {
                        break;
                    }
                    k -= 1;
                }
                ret += j;
            }
            if i != 1 {
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
        let mut input_nums: Vec<i32> = Vec::new();
        for s in input.split_whitespace() {
            input_nums.push(s.parse().unwrap())
        }
        let m = input_nums[0];
        let n = input_nums[1];
        let k = input_nums[2];


        println!("{}", Solution::find_kth_number(m, n, k));
    }
}
