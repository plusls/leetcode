use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut max_count = 0_usize;
        Self::dfs(&root, &mut map, &mut max_count);
        let mut ret = Vec::new();
        // println!("{:?}", map);
        for (k, v) in map {
            if v == max_count {
                ret.push(k);
            }
        }
        ret
    }
    pub fn dfs(tree_node: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, usize>, max_count: &mut usize) -> i32 {
        use std::cmp::max;
        use std::collections::hash_map::Entry;

        if let Some(tree_node) = tree_node {
            let tree_node = tree_node.borrow();
            let left = Self::dfs(&tree_node.left, map, max_count);
            let right = Self::dfs(&tree_node.right, map, max_count);
            match map.entry(left + right + tree_node.val) {
                Entry::Occupied(mut occupied) => {
                    let new_val = occupied.get() + 1;
                    occupied.insert(new_val);
                    *max_count = max(*max_count, new_val);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(1);
                    *max_count = max(*max_count, 1);
                }
            }
            left + right + tree_node.val
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;


fn build_tree(nums: &Vec<Option<i32>>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if idx >= nums.len() {
        return None;
    }
    let ret = Rc::new(RefCell::new(TreeNode::new(nums[idx]?)));
    let mut ret_mut = ret.borrow_mut();
    ret_mut.left = build_tree(nums, idx * 2 + 1);
    ret_mut.right = build_tree(nums, idx * 2 + 2);
    drop(ret_mut);
    Some(ret)
}


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut nums: Vec<Option<i32>> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            if s == "null" {
                nums.push(None);
            } else {
                nums.push(Some(s.parse().unwrap()));
            }
        }

        println!("{:?}", Solution::find_frequent_tree_sum(build_tree(&nums, 0)));
    }
}