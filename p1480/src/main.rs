impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![0; nums.len()];
        for (i, num) in nums.iter().enumerate() {
            ret[i] = if i != 0 { ret[i - 1] } else { 0 } + num;
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

        println!("{:?}", Solution::running_sum(nums));
    }
}