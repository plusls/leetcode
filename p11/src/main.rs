use std::io;
use std::cmp::{max, min};


impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ret = 0;
        while l < r {
            let h_l = height[l];
            let h_r = height[r];
            ret = max(ret, min(h_l, h_r) * ((r - l) as i32));
            if height[l] < height[r] {
                while l < r {
                    l += 1;
                    if height[l] > h_l {
                        break;
                    }
                }
            } else {
                while l < r {
                    r -= 1;
                    if height[r] > h_r {
                        break;
                    }
                }
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
        let mut height = Vec::new();
        for n in input.split_whitespace() {
            height.push(n.parse().unwrap());
        }

        let result = Solution::max_area(height);
        println!("{}", result);
    }
}
