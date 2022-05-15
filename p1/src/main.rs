use std::io;
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums.iter().enumerate().map(|(i, v)| (i, *v)).collect();
        nums.sort_unstable_by_key(|(_, v)| *v);


        let mut result = Vec::new();
        let mut l = 0;
        let mut r = nums.len();
        loop {
            // why?
            // let nums = &nums;
            assert!(l < r);
            if nums[l].1 + nums[r - 1].1 < target {
                l += 1;
                continue;
            }
            while nums[l].1 + nums[r - 1].1 > target {
                r -= 1;
                assert!(r > l);
            }

            let idx = Solution::find_target_idx(&nums, target - nums[l].1, l, r);
            if idx != -1 {
                result.push(nums[l].0 as i32);
                result.push(idx);
                return result;
            }
        }
    }

    pub fn find_target_idx(nums: &[(usize, i32)], target: i32, mut l: usize, mut r: usize) -> i32 {
        assert!(l < r);
        while l < r {
            let mid = (l + r) / 2;
            match nums[mid].1.cmp(&target) {
                Ordering::Less => {
                    if l == mid {
                        break;
                    }
                    l = mid + 1;
                    if l >= r {
                        break;
                    }
                    continue;
                }
                Ordering::Greater => {
                    r = mid;
                }
                Ordering::Equal => {
                    return nums[mid].0 as i32;
                }
            }
        }
        -1
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
        let mut nums = Vec::new();
        for n in input.split_whitespace() {
            nums.push(n.parse().unwrap());
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut input = input.split_whitespace();
        let target: i32 = input.next().unwrap().parse().unwrap();
        let result = Solution::two_sum(nums, target);
        println!("{} {}", result[0], result[1]);
    }
}
