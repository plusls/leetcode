use std::io;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut i = 0;
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 4 {
            return ret;
        }
        nums.sort_unstable();
        // println!("nums:{:?}", nums);

        while i < nums.len() - 3 {
            let r_3 = target - nums[i];
            let mut j = i + 1;
            if r_3 < nums[j] && nums[j] >= 0 {
                break;
            }
            while j < nums.len() - 2 {
                let r_2 = r_3 - nums[j];
                let mut k = j + 1;
                if r_2 < nums[k] && nums[k] >= 0 {
                    break;
                }
                let mut l = nums.len() - 1;
                while k < l {
                    // println!("i:{}, j:{}, k:{}, l:{}", i, j, k, l);
                    let sum_k_l = nums[k] + nums[l];
                    if sum_k_l <= r_2 {
                        if sum_k_l == r_2 {
                            ret.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        }
                        k += 1;
                        while k < l && nums[k - 1] == nums[k] {
                            k += 1;
                        }
                    } else {
                        l -= 1;
                    }
                }
                j += 1;
                while j < nums.len() - 2 && nums[j - 1] == nums[j] {
                    j += 1;
                }
            }
            i += 1;
            while i < nums.len() - 3 && nums[i - 1] == nums[i] {
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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let target = input.parse().unwrap();
        println!("{:?}", Solution::four_sum(nums, target));
    }
}
