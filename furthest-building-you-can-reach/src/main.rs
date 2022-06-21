impl Solution {
    pub fn furthest_building(mut heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        use std::collections::BinaryHeap;

        for i in 0..heights.len() - 1 {
            let hi = heights[i];
            let hi_1 = heights[i + 1];
            heights[i] = if hi_1 <= hi {
                0
            } else {
                hi_1 - hi
            };
        }
        heights.pop();
        // println!("heights:{:?}", heights);
        let mut heap = BinaryHeap::new();
        for (i, &height) in heights.iter().enumerate() {
            if height > bricks {
                if ladders == 0 {
                    return i as i32;
                }
                if !heap.is_empty() {
                    if height > *heap.peek().unwrap() {
                        ladders -= 1;
                    } else {
                        bricks += heap.pop().unwrap() - height;
                        heap.push(height);
                        ladders -= 1;
                    }
                } else {
                    ladders -= 1;
                }
            } else if height != 0 {
                heap.push(height);
                bricks -= height;
            }
            // println!("i:{}, height:{}, ladders:{}, bricks:{}, heap:{:?}", i, height, ladders, bricks, heap);

            if i == heights.len() - 1 {
                return i as i32 + 1;
            }
        }
        0
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
        let mut heights: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            heights.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let bricks: i32 = input.trim().parse().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let ladders: i32 = input.trim().parse().unwrap();

        println!("{:?}", Solution::furthest_building(heights, bricks, ladders));
    }
}
