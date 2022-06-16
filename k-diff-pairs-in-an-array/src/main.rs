impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        // println!("nums:{:?}, k:{}", nums, k);

        let mut map: HashMap<i32, bool> = HashMap::new();
        let mut ret = 0;
        for num in nums {
            // println!("num:{}, map:{:?}", num, map);
            if let Entry::Occupied(mut occupied) = map.entry(num + k) {
                // println!("num:{}, num + k:{}", num, num + k);
                if !occupied.get() {
                    occupied.insert(true);
                    ret += 1;
                }
            }
            if map.contains_key(&(num - k)) {
                match map.entry(num) {
                    Entry::Occupied(mut occupied) => {
                        if !occupied.get() {
                            occupied.insert(true);
                            ret += 1;
                        }
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(true);
                        ret += 1;
                    }
                }
            } else {
                map.entry(num).or_insert(false);
            }
        }
        // println!("map:{:?}", map);
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
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", Solution::find_pairs(nums, input.trim().parse().unwrap()));
    }
}
