impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut flag = false;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                if flag {
                    return false;
                } else {
                    flag = true;
                    if i + 2 == nums.len() {
                        return true;
                    }
                    if nums[i] > nums[i + 2] {
                        if i == 0 || nums[i - 1] <= nums[i + 1] {
                            nums[i] = nums[i + 1];
                        } else {
                            return false;
                        }
                    } else {
                        nums[i + 1] = nums[i + 2];
                    }
                }
            }
        }
        true
    }
}

struct Solution;


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim().replace(' ', "");
        let mut nums: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }
        println!("{:?}", Solution::check_possibility(nums));
    }
}
