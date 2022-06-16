impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut last_idx = arr.len() - 1;
        let mut last_skip = false;
        for i in 0..arr.len() {
            if i >= last_idx {
                if i == last_idx && arr[last_idx] == 0 {
                    last_skip = true;
                }
                break;
            }
            if arr[i] == 0 {
                last_idx -= 1;
            }
        }
        if last_idx == arr.len() - 1 {
            return;
        }
        let mut current_idx = last_idx;
        let mut idx = arr.len() - 1;
        loop {
            let n = arr[current_idx];
            arr[idx] = n;
            if n == 0 && !(current_idx == last_idx && last_skip) {
                idx -= 1;
                arr[idx] = 0;
            }
            if idx == 0 {
                break;
            }
            // println!("idx: {}, current_idx: {}, {:?}", idx, current_idx, arr);
            idx -= 1;
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
        let mut arr: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            arr.push(s.parse().unwrap());
        }
        Solution::duplicate_zeros(&mut arr);
        println!("{:?}", arr);
    }
}
