use std::collections::HashMap;

struct UnionFind {
    parent: HashMap<i32, i32>,
    size: HashMap<i32, i32>,
    max_size: usize,
}

impl UnionFind {
    pub fn new() -> Self {
        Self { parent: HashMap::new(), size: HashMap::new(), max_size: 0 }
    }

    pub fn union(&mut self, value: i32, parent: i32) {
        // println!("union: {} {}", value, parent);
        // println!("parent: {:?}", self.parent);
        // println!("size: {:?}", self.size);
        if value == parent {
            if let std::collections::hash_map::Entry::Vacant(e) = self.parent.entry(value) {
                e.insert(value);
                self.size.insert(value, 1);
                self.max_size = self.max_size.max(1);
            }
        } else {
            let parent_parent = self.find(parent).unwrap();
            let value_parent = self.find(value);
            if let Some(value_parent) = value_parent {
                if value_parent == parent_parent {
                    return;
                }
                // println!("value_parent: {} parent_parent: {}", value_parent, parent_parent);
                let value_parent_size = self.size.remove(&value_parent).unwrap();
                self.parent.insert(value_parent, parent_parent);
                let size = self.size.get_mut(&parent_parent).unwrap();
                *size += value_parent_size;
                self.max_size = self.max_size.max(*size as usize);
            } else {
                self.parent.insert(value, parent_parent);
                let size = self.size.get_mut(&parent_parent).unwrap();
                *size += 1;
                self.max_size = self.max_size.max(*size as usize);
            }
        }
    }
    pub fn find(&mut self, value: i32) -> Option<i32> {
        let mut parent = *self.parent.get(&value).unwrap();
        loop {
            let &next = self.parent.get(&parent).unwrap();
            if next == parent {
                self.parent.insert(value, parent);
                return Some(next);
            }
            parent = next;
        }
    }
}


impl Solution {
    pub fn update_map(n: i32, map: &mut HashMap<i32, i32>, union_find: &mut UnionFind) {
        let mut factors = Vec::new();
        if n == 1 {
            factors.push(1);
        }
        let mut new_n = n;
        for i in 2..(n as f64).sqrt() as i32 + 2 {
            if new_n % i == 0 {
                factors.push(i);
                while new_n % i == 0 {
                    new_n /= i;
                }
            }
            if i > new_n {
                break;
            }
        }
        if new_n != 1 {
            factors.push(new_n);
        }
        union_find.union(n, n);
        // println!("factors: {:?}", factors);
        for factor in factors {
            match map.entry(factor) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(n);
                }
                std::collections::hash_map::Entry::Occupied(entry) => {
                    let n_parent = union_find.find(n).unwrap();
                    union_find.union(n_parent, *entry.get());
                }
            }
        }
        // println!("map: {:?}", map);
    }

    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut union_find = UnionFind::new();
        let mut map = HashMap::new();
        for n in nums {
            Self::update_map(n, &mut map, &mut union_find);
        }
        union_find.max_size as i32
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
        println!("{:?}", Solution::largest_component_size(nums));
    }
}
