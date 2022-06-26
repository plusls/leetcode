impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut current = card_points[0..k as usize].iter().sum::<i32>();
        let mut ret = current;
        let n = card_points.len();
        for i in 0..k as usize {
            current = current - card_points[k as usize - i - 1] + card_points[n - i - 1];
            ret = ret.max(current);
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
        let input = input.trim().replace(' ', "");
        let mut card_points: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            card_points.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", Solution::max_score(card_points, input.trim().parse().unwrap()));
    }
}
