use std::io;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // println!("nums: {:?}", nums);
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let array_len = if n % 8 == 0 { n / 8 } else { n / 8 + 1 };
        let mut numbers: Vec<u8> = vec![0; array_len];
        for num in nums {
            let idx = num / 8;
            let off = num % 8;
            numbers[idx as usize] |= 1 << off;
        }
        for (i, number) in numbers.iter().enumerate() {
            let mut number = *number;
            if number != 0xff {
                let mut j = 0;
                while j < 8 {
                    if number & 1 == 0 {
                        return i as i32 * 8 + j;
                    }
                    j += 1;
                    number >>= 1;
                }
            }
        }
        // println!("numbers: {:?}", numbers);
        unreachable!()
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for n in input.split(",") {
            let n = n.replace(' ', "").replace('[', "").replace(']', "");
            if !n.is_empty() {
                nums.push(n.parse().unwrap());
            } else {
                break;
            }
        }
        println!("{}", Solution::missing_number(nums));
    }
}
