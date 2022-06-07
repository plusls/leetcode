impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }
        if m == 0 {
            for (i, num) in nums1.iter_mut().enumerate() {
                *num = nums2[i];
            }
            return;
        }
        let mut current_idx = nums1.len() - 1;
        let mut idx_1 = m - 1;
        let mut idx_2 = n - 1;
        let mut num1 = nums1[idx_1 as usize];
        let mut num2 = nums2[idx_2 as usize];

        loop {
            nums1[current_idx] = if num1 > num2 {
                let tmp = num1;
                if idx_1 > 0 {
                    idx_1 -= 1;
                    num1 = nums1[idx_1 as usize];
                } else {
                    num1 = i32::MIN;
                }
                tmp
            } else {
                let tmp = num2;
                if idx_2 > 0 {
                    idx_2 -= 1;
                    num2 = nums2[idx_2 as usize];
                } else {
                    num2 = i32::MIN;
                }
                tmp
            };
            if current_idx == 0 {
                return;
            }
            current_idx -= 1;
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
        let mut nums1: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            if s.is_empty() {
                break;
            }
            nums1.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut nums2: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            if s.is_empty() {
                break;
            }
            nums2.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n = nums2.len();
        Solution::merge(&mut nums1, input.trim().parse().unwrap(), &mut nums2, n as i32);
        println!("{:?}", nums1);
    }
}