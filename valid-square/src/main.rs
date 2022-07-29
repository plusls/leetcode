impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let points = vec![p1.clone(), p2, p3, p4];
        let mut edge1_vec = vec![0_usize];
        let mut max_dist = i32::MIN;
        let mut max_dist_index = 0_usize;
        for (i, point) in points.iter().enumerate() {
            let dist = (point[0] - p1[0]).abs().pow(2) + (point[1] - p1[1]).abs().pow(2);
            if dist > max_dist {
                max_dist = dist;
                max_dist_index = i;
            }
        }
        if max_dist == 0 {
            return false;
        }
        edge1_vec.push(max_dist_index);
        let p0 = points[edge1_vec[0]].clone();
        let p1 = points[edge1_vec[1]].clone();
        let mut edge2_vec = vec![0_usize, 1, 2, 3];
        edge2_vec.retain(|&i| i != edge1_vec[0] && i != edge1_vec[1]);
        let p2 = points[edge2_vec[0]].clone();
        let p3 = points[edge2_vec[1]].clone();
        !(p0[0] + p1[0] != p2[0] + p3[0] || p0[1] + p1[1] != p2[1] + p3[1] ||
            (p0[0] - p1[0]).abs().pow(2) + (p0[1] - p1[1]).abs().pow(2) !=
                (p2[0] - p3[0]).abs().pow(2) + (p2[1] - p3[1]).abs().pow(2) ||
            (p0[0] - p1[0]) * (p2[0] - p3[0]) + (p0[1] - p1[1]) * (p2[1] - p3[1]) != 0)
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
        let input = input.trim().replace(' ', "");
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
        println!("{:?}", Solution::valid_square(points[0].clone(), points[1].clone(),
                                                points[2].clone(), points[3].clone()));
    }
}
