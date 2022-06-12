impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        use std::collections::HashMap;
        use std::cmp::min;
        // println!("nums: {:?}, x: {}", nums, x);
        let mut sums: Vec<i32> = Vec::new();
        let mut sum = 0;
        let mut sums_idx_map: HashMap<i32, i32> = HashMap::new();
        sums_idx_map.insert(0, -1);
        for num in &nums {
            sum += num;
            sums.push(sum);
            sums_idx_map.insert(sum, sums.len() as i32 - 1);
        }
        let mut result = i32::MAX;
        // println!("sums: {:?}", sums);
        for i in (0..nums.len()).rev() {
            let n = x - (sum - sums[i]);
            if let Some(idx) = sums_idx_map.get(&n) {
                let idx = *idx;
                if idx > i as i32 {
                    continue;
                }
                result = min(result, nums.len() as i32 - 1 - i as i32 + idx + 1);
                // println!("left: {}, right: {}", idx + 1, nums.len() - 1 - i);
            }
        }

        if result == i32::MAX {
            -1
        } else {
            result
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
