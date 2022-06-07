impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x1 = points[0][0] - points[1][0];
        let y1 = points[0][1] - points[1][1];
        let x2 = points[1][0] - points[2][0];
        let y2 = points[1][1] - points[2][1];
        x1 * y2 != x2 * y1
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
        let mut points: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            points.push(Vec::new());
            let l = points.len();
            for column in row.split(',') {
                points[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::is_boomerang(points));
    }
}
