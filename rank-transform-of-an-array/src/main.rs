impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;

        let mut arr1 = arr.clone();
        let mut map = HashMap::new();

        arr1.sort_unstable();
        let mut idx = 1;
        arr1.iter().for_each(|&x| {
            if let Entry::Vacant(e) = map.entry(x) {
                e.insert(idx);
                idx += 1;
            }
        });
        arr.iter().map(|x| *map.get(x).unwrap()).collect::<Vec<i32>>()
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

        println!("{:?}", Solution::array_rank_transform(nums));
    }
}
