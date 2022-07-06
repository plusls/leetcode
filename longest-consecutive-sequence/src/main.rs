impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = nums.iter().copied().collect();
        let mut ret = 0;
        for num in nums {
            let mut l_count = 0;
            while set.remove(&(num - l_count - 1)) {
                l_count += 1;
            }
            let mut r_count = 0;
            while set.remove(&(num + r_count)) {
                r_count += 1;
            }
            ret = ret.max(l_count + r_count);
        }
        ret
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

        println!("{:?}", Solution::longest_consecutive(nums));
    }
}
