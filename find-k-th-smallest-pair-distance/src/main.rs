impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, x: i32) -> i32 {
        nums.sort_unstable();
        let (mut l, mut r) = (0, nums[nums.len() - 1] - nums[0] + 1);
        // println!("nums: {:?}", nums);
        loop {
            let mid = (l + r) / 2;
            let (cnt_l, cnt_r, ret) = Self::calc_cnt(&nums, mid);
            if cnt_r < x {
                l = mid + 1;
            } else if cnt_l >= x {
                r = mid;
            } else {
                return ret;
            }
        }
    }

    pub fn calc_cnt(nums: &[i32], v: i32) -> (i32, i32, i32) {
        use std::cmp::max;
        let mut cnt_l = 0_i32;
        let mut cnt_r = 0_i32;
        let mut j_l = 0_usize;
        let mut j_r = 0_usize;
        let mut ret = 0_i32;
        for (i, num) in nums.iter().enumerate() {
            loop {
                ret = max(ret, nums[j_r] - num);
                if j_r == nums.len() - 1 || nums[j_r + 1] - num > v {
                    break;
                }
                j_r += 1;
            }
            loop {
                if j_l == nums.len() - 1 || nums[j_l + 1] - num >= v {
                    break;
                }
                j_l += 1;
            }
            // cnt_l += (j_l - i) as i32;
            cnt_l += if j_l >= i { (j_l - i) as i32 } else { 0 };
            cnt_r += (j_r - i) as i32;
            // println!("i: {}, j_l: {}, j_r: {}, cnt_l: {} cnt_r: {}", i, j_l, j_r, cnt_l, cnt_r);
        }
        // println!("calc_cnt(v: {}) -> ({}, {}, {})", v, cnt_l, cnt_r, ret);
        (cnt_l, cnt_r, ret)
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

        println!("{:?}", Solution::smallest_distance_pair(nums, input.trim().parse().unwrap()));
    }
}
