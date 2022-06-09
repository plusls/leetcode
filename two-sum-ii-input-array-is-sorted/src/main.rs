impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp;
        let mut l = 0;
        let mut r = numbers.len() - 1;
        loop {
            let n_l = numbers[l];
            let n_r = numbers[r];
            match (n_l + n_r).cmp(&target) {
                cmp::Ordering::Less => {
                    l += 1;
                }
                cmp::Ordering::Equal => {
                    return vec![l as i32 + 1, r as i32 + 1];
                }
                cmp::Ordering::Greater => {
                    r -= 1;
                }
            }
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
        let mut numbers: Vec<i32> = Vec::new();
        for n in input.split(',') {
            numbers.push(n.replace('[', "").replace(']', "").trim().parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{:?}", Solution::two_sum(numbers, input.parse().unwrap()));
    }
}
