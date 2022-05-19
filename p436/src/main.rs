use std::cmp;
use std::io;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut new_intervals: Vec<(i32, i32, usize)> = Vec::new();
        for (i, interval) in intervals.iter().enumerate() {
            new_intervals.push((interval[0], interval[1], i));
        }
        new_intervals.sort_unstable();
        let mut ret = vec![-1; new_intervals.len()];

        for interval in &new_intervals {
            ret[interval.2] = Self::search(&new_intervals, interval.1);
        }
        ret
    }

    pub fn search(new_intervals: &[(i32, i32, usize)], v: i32) -> i32 {
        let mut l = 0_usize;
        let mut r = new_intervals.len();
        let mut ret: i32 = -1;
        while l < r {
            let mid = (l + r) / 2;
            match new_intervals[mid].0.cmp(&v) {
                cmp::Ordering::Less => {
                    l = mid + 1;
                }
                cmp::Ordering::Equal => {
                    ret = new_intervals[mid].2 as i32;
                    break;
                }
                cmp::Ordering::Greater => {
                    ret = new_intervals[mid].2 as i32;
                    r = mid;
                }
            }
        }
        ret
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
        let mut intervals: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            intervals.push(Vec::new());
            let l = intervals.len();
            for column in row.split(',') {
                intervals[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::find_right_interval(intervals));
    }
}
