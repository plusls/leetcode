use std::io;
use std::{cmp, mem};


impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;

        if nums1.len() > nums2.len() {
            mem::swap(&mut nums1, &mut nums2);
        }

        let m = nums1.len();
        let n = nums2.len();

        let count = m + n;
        let mid_size = (count as f32 / 2.0).ceil() as usize;


        let mut l = cmp::max(0, mid_size as i32 - n as i32) as usize;
        let mut r = cmp::min(m, mid_size) + 1;
        let mut last_l = l;
        loop {
            let i = (l + r) / 2;
            let j = mid_size - i;
            // println!("i:{} j:{} l:{} r:{} mid_size:{}", i, j, l, r, mid_size);
            let nums1_i_1 = Solution::get_nums_data(&nums1, (i - 1) as i32);
            let nums2_j = Solution::get_nums_data(&nums2, j as i32);

            if nums1_i_1 <= nums2_j {
                if l == i {
                    break;
                }
                last_l = l;
                l = i + 1;
                if l == r {
                    l -= 1;
                    break;
                }
            } else {
                r = i;
                if l == r {
                    l = last_l;
                }
            }
        }
        let i = l;
        let j = mid_size - i;

        assert!(Solution::get_nums_data(&nums1, i as i32 - 1) <= Solution::get_nums_data(&nums2, j as i32));
        assert!(Solution::get_nums_data(&nums2, j as i32 - 1) <= Solution::get_nums_data(&nums1, i as i32));

        let l_n = cmp::max(Solution::get_nums_data(&nums1, i as i32 - 1), Solution::get_nums_data(&nums2, j as i32 - 1));
        let r_n = cmp::min(Solution::get_nums_data(&nums1, i as i32), Solution::get_nums_data(&nums2, j as i32));
        if count % 2 == 0 {
            (l_n + r_n) as f64 / 2.0
        } else {
            l_n as f64
        }
    }

    fn get_nums_data(nums: &[i32], idx: i32) -> i32 {
        assert!(idx >= -1);
        if idx == -1 {
            return i32::MIN;
        }
        let idx = idx as usize;
        assert!(idx <= nums.len());

        if idx == nums.len() { i32::MAX } else { nums[idx] }
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
        let mut nums1 = Vec::new();
        for n in input.split_whitespace() {
            nums1.push(n.parse().unwrap());
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut nums2 = Vec::new();
        for n in input.split_whitespace() {
            nums2.push(n.parse().unwrap());
        }

        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        println!("{}", result);
    }
}
