use std::io;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // println!("x:{}", x);
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        if x % 10 == 0 {
            return false;
        }
        let mut x = x;
        let mut y = 0;
        while y < x {
            y *= 10;
            y += x % 10;
            x /= 10;
        }
        // println!("x:{} y:{}", x, y);
        y == x || (y / 10 == x && x != 0)
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
        println!("{}", Solution::is_palindrome(input.parse().unwrap()));
    }
}
