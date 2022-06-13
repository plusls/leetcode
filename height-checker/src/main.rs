impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut new_heights = heights.clone();
        new_heights.sort_unstable();
        let mut ret = 0;
        for (i, height) in heights.iter().enumerate() {
            if *height != new_heights[i] {
                ret += 1;
            }
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
        let mut heights: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            heights.push(s.parse().unwrap());
        }

        println!("{:?}", Solution::height_checker(heights));
    }
}
