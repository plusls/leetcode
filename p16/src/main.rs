use std::cmp::Ordering;
use std::io;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut i = 0;
        let mut ret = i32::MAX;
        nums.sort_unstable();
        // println!("nums:{:?} target:{}", nums, target);

        while i < nums.len() - 2 && ret != target {
            let r_3 = target - nums[i];

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum_j_k = nums[j] + nums[k];
                let sum_i_j_k = nums[i] + sum_j_k;
                // println!("i:{}, j:{}, k:{} ret:{} {}", i, j, k, ret, (sum_i_j_k - target).abs() < (ret - target).abs());

                if ret == i32::MAX || (sum_i_j_k - target).abs() < (ret - target).abs() {
                    ret = sum_i_j_k;
                }
                match sum_j_k.cmp(&r_3) {
                    Ordering::Less => {
                        j += 1;
                        while j < k && nums[j - 1] == nums[j] {
                            j += 1;
                        }
                    }
                    Ordering::Equal => {
                        ret = target;
                        break;
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        println!("{}", Solution::three_sum_closest(nums, input.parse().unwrap()));
    }
}
