use std::io;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        nums.iter().for_each(|num| sum += num);
        if sum % k != 0 {
            return false;
        }
        let edge_len = sum / k;
        let mut dp: Vec<Option<bool>> = vec![None; 1 << nums.len()];
        // 结束条件
        dp[(1 << nums.len()) - 1] = Some(true);
        Self::dfs(0, 0, edge_len, &nums, &mut dp)
    }

    pub fn dfs(current: usize, current_edge_len: i32, edge_len: i32, nums: &[i32], dp: &mut [Option<bool>]) -> bool {
        if let Some(result) = dp[current] {
            return result;
        }
        for (i, num) in nums.iter().enumerate() {
            let new_edge_len = current_edge_len + num;
            // println!("current:{:b}, current_edge_len:{}, new_edge_len:{}, edge_len:{}", current, current_edge_len, new_edge_len, edge_len);
            if (current >> i) & 1 == 0 && new_edge_len <= edge_len {
                // println!("current:{:b}, current_edge_len:{}, new_edge_len:{}, edge_len:{}", current, current_edge_len, new_edge_len, edge_len);
                if Self::dfs(current | (1 << i), new_edge_len % edge_len, edge_len, nums, dp) {
                    return true;
                }
            }
        }
        dp[current] = Some(false);
        false
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let mut nums: Vec<i32> = Vec::new();
        for s in input.trim().split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::can_partition_k_subsets(nums, input.trim().parse().unwrap()));
    }
}
