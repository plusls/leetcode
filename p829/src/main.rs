use std::io;

impl Solution {
    pub fn consecutive_numbers_sum(mut n: i32) -> i32 {
        while n % 2 == 0 {
            n /= 2;
        }
        if n == 1 {
            return 1;
        }
        let mut ret = 0;
        let mut i = 1;
        let max_i = (n as f32).sqrt().ceil() as i32;
        while i <= max_i {
            if n % i == 0 {
                if n != i * i {
                    ret += 2
                } else {
                    ret += 1;
                    break;
                }
            }
            i += 2;
        }
        ret
    }
    pub fn consecutive_numbers_sum_correct(n: i32) -> i32 {
        // println!("n:{}",n);
        let mut ret = 0;
        let n = n as i128;
        for i in 1..(n + 1) {
            let delta = 1 - 4 * i + 4 * i * i + 8 * n;
            if delta >= 1 {
                let y = ((delta as f64).sqrt().round() as i128 - 1) / 2;
                if y * y - i * i + i + y == 2 * n {
                    // println!("x:{}, y:{}, n:{}", i, y, n);
                    ret += 1;
                }
            }
        }
        ret
    }
}


struct Solution();


fn main() {
    for i in 1..1000 {
        assert_eq!(Solution::consecutive_numbers_sum(i), Solution::consecutive_numbers_sum_correct(i));
    }
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();

        println!("{:?}", Solution::consecutive_numbers_sum(input.parse().unwrap()));
    }
}
