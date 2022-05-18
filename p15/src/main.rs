use std::io;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut i = 0;
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 3 {
            return ret;
        }
        nums.sort_unstable();
        // println!("nums:{:?}", nums);

        while i < nums.len() - 2 {
            let r_3 = -nums[i];
            if r_3 < 0 {
                break;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                // println!("i:{}, j:{}, k:{}", i, j, k);
                let sum_j_k = nums[j] + nums[k];
                if sum_j_k <= r_3 {
                    if sum_j_k == r_3 {
                        ret.push(vec![nums[i], nums[j], nums[k]]);
                    }
                    j += 1;
                    while j < k && nums[j - 1] == nums[j] {
                        j += 1;
                    }
                } else {
                    k -= 1;
                }
            }
            i += 1;
            while i < nums.len() - 2 && nums[i - 1] == nums[i] {
                i += 1;
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
        let mut nums: Vec<i32> = Vec::new();
        for n in input.split(',') {
            let n = n.replace('[', "").replace(']', "");
            if !n.is_empty() {
                nums.push(n.parse().unwrap());
            }
        }
        println!("{:?}", Solution::three_sum(nums));
    }
}
