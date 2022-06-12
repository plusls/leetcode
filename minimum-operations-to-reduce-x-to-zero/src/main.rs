impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        use std::cmp::min;
        // println!("nums: {:?}, x: {}", nums, x);
        let target = nums.iter().sum::<i32>() - x;
        let mut l = 0;
        let mut r = 0;
        let len = nums.len();
        let mut sum = 0;
        let mut ret = i32::MAX;
        while r < len {
            sum += nums[r];
            while sum > target && l <= r {
                sum -= nums[l];
                l += 1;
            }
            if sum == target {
                ret = min(ret, (len - (r + 1 - l)) as i32);
            }
            // println!("l:{}, r:{}, sum:{}, target:{}", l, r, sum, target);
            r += 1;
        }

        if ret == i32::MAX {
            -1
        } else {
            ret
        }
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
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::min_operations(nums, input.trim().parse().unwrap()));
    }
}
