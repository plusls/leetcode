impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 0 {
            (0..n - 1).map(|_| 'a').collect::<String>() + "b"
        } else {
            (0..n).map(|_| 'a').collect()
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


        println!("{:?}", Solution::generate_the_string(input.trim().parse().unwrap()));
    }
}
