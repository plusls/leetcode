impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();
        let mut sum = 0_i64;
        let n = target.len() as i64;
        for num in target {
            heap.push(num as i64);
            sum += num as i64;
        }
        while *heap.peek().unwrap() > 1 {
            let v = heap.pop().unwrap();
            let v1 = *heap.peek().unwrap();
            let diff = sum - v as i64;
            if diff != 0 {
                let sub = ((v - v1) / diff + 1) * diff;
                let mut prev_v = v - sub;
                sum -= sub;

                // println!("prev_v: {}, sum:{}", prev_v, sum);
                if prev_v < 1 {
                    if prev_v + diff == 1 {
                        prev_v += diff;
                        sum += diff;
                    } else {
                        return false;
                    }
                }
                heap.push(prev_v);
                // println!("heap:{:?}", heap);
            } else {
                return false;
            }
        }
        sum == n
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
        let mut target: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            target.push(s.parse().unwrap());
        }

        println!("{:?}", Solution::is_possible(target));
    }
}
