impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;
        while num > 0 {
            num = if num % 2 == 0 {
                num / 2
            } else {
                num - 1
            };
            ret += 1;
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

        println!("{:?}", Solution::number_of_steps(input.parse().unwrap()));
    }
}