impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        courses.sort_unstable_by(|a, b| a.iter().rev().cmp(b.iter().rev()));
        // println!("courses: {:?}", courses);
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut ret = 0;
        let mut current_time = 0;
        for course in courses {
            // println!("course: {:?}, current_time: {:?}", course, current_time);
            if current_time + course[0] <= course[1] {
                current_time += course[0];
                heap.push((course[0], course[1]));
                ret += 1
            } else if let Some(v) = heap.peek() {
                if v.0 >= course[0] {
                    current_time -= v.0 - course[0];
                    heap.pop();
                    heap.push((course[0], course[1]));
                }
            }
        }
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
        let mut courses: Vec<Vec<i32>> = Vec::new();
        for row in input.replace(' ', "").split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            courses.push(Vec::new());
            let l = courses.len();
            for column in row.split(',') {
                courses[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::schedule_course(courses));
    }
}
