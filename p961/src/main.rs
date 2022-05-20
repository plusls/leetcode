use std::io;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() - 2 {
            if nums[i] == nums[i + 1] || nums[i] == nums[i + 2] {
                return nums[i];
            }
        }
        nums[nums.len() - 1]
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
        let mut nums: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }

        println!("{:?}", Solution::repeated_n_times(nums));
    }
}
