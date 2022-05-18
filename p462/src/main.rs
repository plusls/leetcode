use std::io;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mid = if nums.len() % 2 == 0 {
            (nums[nums.len() / 2] + nums[nums.len() / 2 - 1]) / 2
        } else {
            nums[(nums.len() - 1) / 2]
        };
        let mut ret = 0;
        for n in nums {
            ret += (n - mid).abs();
        }
        ret
    }
}


struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input = input.trim();
        let mut nums = Vec::new();
        for n in input.split(',') {
            let n = n.replace('[', "").replace(']', "");
            nums.push(n.parse().unwrap());
        }

        println!("{:?}", Solution::min_moves2(nums));
    }
}
