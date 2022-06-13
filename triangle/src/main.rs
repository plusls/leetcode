impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let mut dp: Vec<i32> = vec![i32::MAX; triangle.len()];
        dp[0] = triangle[0][0];
        let mut ret = i32::MAX;
        for t in triangle[1..].iter() {
            for (j, n) in t.iter().rev().enumerate() {
                let j = t.len() - j - 1;
                dp[j] = min(dp[j], if j == 0 { i32::MAX } else { dp[j - 1] }) + n;
            }
        }
        for dp in dp {
            ret = min(ret, dp);
        }
        // println!("{:?}", dp);
        ret
    }
}

struct Solution();


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            triangle.push(Vec::new());
            let l = triangle.len();
            for column in row.split(',') {
                triangle[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::minimum_total(triangle));
    }
}
