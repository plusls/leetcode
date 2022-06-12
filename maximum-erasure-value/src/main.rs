impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut sums: Vec<i32> = Vec::new();
        let mut sum = 0;
        for num in &nums {
            sum += num;
            sums.push(sum);
        }
        let mut max_lens: Vec<usize> = Vec::new();
        let mut idx = 0;
        let mut len = 0;
        let mut current_nums = [false; 10001];
        while idx < nums.len() {
            while idx + len < nums.len() {
                let current_idx = idx + len;
                if current_nums[nums[current_idx] as usize] {
                    break;
                }
                current_nums[nums[current_idx] as usize] = true;
                len += 1;
            }
            max_lens.push(len);
            len -= 1;
            current_nums[nums[idx] as usize] = false;
            idx += 1;
        }
        let mut result = 0;
        for (i, max_len) in max_lens.iter().enumerate() {
            result = max(result, sums[i + max_len - 1] - if i > 0 { sums[i - 1] } else { 0 });
        }
        // println!("max_lens: {:?}", max_lens);
        result
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

        println!("{:?}", Solution::maximum_unique_subarray(nums));
    }
}
