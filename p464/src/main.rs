use std::io;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        let mut dp: Vec<Option<bool>> = vec![None; 1 << max_choosable_integer];
        Self::dfs(max_choosable_integer, 0, &mut dp, desired_total as usize)
    }
    // 10 11
    // 10 0
    // 10 1

    pub fn dfs(max_choosable_integer: i32, status: usize, dp: &mut Vec<Option<bool>>, desired_total: usize) -> bool {
        // println!("status:{:b} dp:{:?}, desired_total:{}", status, dp, desired_total);
        if let Some(ret) = dp[status] {
            return ret;
        }
        for i in (0..max_choosable_integer as usize).rev() {
            // println!("i:{}", i);
            if (status >> i) & 1 == 0 {
                if desired_total <= i + 1 {
                    dp[status] = Some(true);
                    return true;
                } else {
                    let current_status = status | (1 << i);
                    let ret = !Self::dfs(max_choosable_integer, current_status, dp, desired_total - i - 1);
                    if ret {
                        dp[status] = Some(true);
                        return true;
                    }
                }
            }
        }
        dp[status] = Some(false);
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
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for n in input.split_whitespace() {
            nums.push(n.trim().parse().unwrap());
        }
        println!("{}", Solution::can_i_win(nums[0], nums[1]));
    }
}
