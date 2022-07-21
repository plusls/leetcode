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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        let mut root_var = root.as_ref().unwrap().borrow_mut();
        root_var.left = Self::prune_tree(root_var.left.clone());
        root_var.right = Self::prune_tree(root_var.right.clone());

        if root_var.left.is_none() &&
            root_var.right.is_none() &&
            root_var.val == 0
        {
            return None;
        }
        drop(root_var);

        root
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

        println!("{:?}", Solution::prune_tree(build_tree(&nums, 0)));
    }
}