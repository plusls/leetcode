use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::do_sum_root_to_leaf(&root, 0)
    }
    pub fn do_sum_root_to_leaf(root: &Option<Rc<RefCell<TreeNode>>>, prefix: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        // Self::sum_root_to_leaf(root.as_ref().unwrap())
        let root = root.as_ref().unwrap().borrow();
        let prefix = (prefix << 1) | root.val;
        let mut ret = 0;
        ret += Self::do_sum_root_to_leaf(&root.left, prefix);
        ret += Self::do_sum_root_to_leaf(&root.right, prefix);
        if ret == 0 {
            ret = prefix;
        }
        ret
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

        println!("{:?}", Solution::sum_root_to_leaf(build_tree(&nums, 0)));
    }
}