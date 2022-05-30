use std::io;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let flag = (dividend > 0) ^ (divisor > 0);
        // println!("dividend: {}, divisor: {}", dividend, divisor);
        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let mut ret = 0;
        while dividend <= divisor {
            let mut tmp = divisor;
            for i in 0..32 {
                let new_tmp = tmp.checked_add(tmp);
                if new_tmp.is_none() || dividend - new_tmp.unwrap() > 0 {
                    dividend -= tmp;
                    ret += (-1) << i;
                    break;
                }
                tmp = new_tmp.unwrap();
            }
        }
        if flag { ret } else if ret == i32::MIN { i32::MAX } else { -ret }
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let mut nums: Vec<i32> = Vec::new();
        for n in input.trim().split_whitespace() {
            nums.push(n.trim().parse().unwrap());
        }
        let dividend: i32 = nums[0];
        let divisor: i32 = nums[1];
        println!("{:?}", Solution::divide(dividend, divisor));
    }
}
