impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut ret = 0;
        for box_type in box_types {
            heap.push((box_type[1], box_type[0]));
        }
        while let Some((w, n)) = heap.pop() {
            if n >= truck_size {
                ret += w * truck_size;
                break;
            } else {
                ret += w * n;
                truck_size -= n;
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
        let mut box_types: Vec<Vec<i32>> = Vec::new();
        for row in input.replace(' ', "").split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            box_types.push(Vec::new());
            let l = box_types.len();
            for column in row.split(',') {
                box_types[l - 1].push(column.parse().unwrap());
            }
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::maximum_units(box_types, input.trim().parse().unwrap()));
    }
}
