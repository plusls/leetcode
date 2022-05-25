use std::io;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let mut d = vec![0; nums.len()];
        let mut ret = 0;
        d[ret] = nums[0];
        for n in &nums[1..] {
            let n = *n;
            match n.cmp(&d[ret]) {
                cmp::Ordering::Greater => {
                    ret += 1;
                    d[ret] = n;
                }
                cmp::Ordering::Less => {
                    let mut l = 0;
                    let mut r = ret + 1;
                    loop {
                        let mid = (l + r) / 2;
                        match n.cmp(&d[mid]) {
                            cmp::Ordering::Greater => {
                                l = mid + 1;
                            }
                            cmp::Ordering::Equal => {
                                break;
                            }
                            cmp::Ordering::Less => {
                                r = mid;
                            }
                        }

                        if l == r {
                            d[l] = n;
                            break;
                        }
                    }
                }
                _ => {}
            }
            // println!("n:{}, d: {:?}", n, d);
        }
        ret as i32 + 1
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
        for n in input.split(',') {
            nums.push(n.replace('[', "").replace(']', "").trim().parse().unwrap());
        }

        println!("{}", Solution::length_of_lis(nums));
    }
}
