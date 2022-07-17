impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                continue;
            }
            let mut cnt = 0;
            let mut idx = i;
            while nums[idx] != -1 {
                let tmp = nums[idx];
                nums[idx] = -1;
                idx = tmp as usize;
                cnt += 1;
            }
            ret = ret.max(cnt);
        }
        ret
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

        println!("{:?}", Solution::array_nesting(nums));
    }
}
