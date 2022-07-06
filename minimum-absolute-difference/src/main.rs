impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut ret = Vec::new();
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        for (i, &b) in arr[1..].iter().enumerate() {
            let a = arr[i];
            let diff = b - a;
            match diff.cmp(&min_diff) {
                Ordering::Less => {
                    min_diff = diff;
                    ret.clear();
                    ret.push(vec![a, b]);
                }
                Ordering::Equal => {
                    ret.push(vec![a, b]);
                }
                Ordering::Greater => {}
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

        println!("{:?}", Solution::minimum_abs_difference(heights));
    }
}