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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        use std::mem;
        if root.is_none() {
            return root;
        }
        let root = root.unwrap();
        let mut root = root.borrow_mut();
        let new_root = Rc::new(RefCell::new(TreeNode::new(root.val)));
        {
            let mut new_root = new_root.borrow_mut();
            new_root.left = mem::replace(&mut root.left, None);
            new_root.right = mem::replace(&mut root.right, None);
        }
        let mut new_root = Some(new_root);
        Self::my_delete_node(&mut new_root, key);
        new_root
    }

    pub fn my_delete_node(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) {
        use std::{cmp, mem};
        if root.is_none() {
            return;
        }
        let root_node = root.as_ref().unwrap();
        let mut root_node = root_node.borrow_mut();
        match root_node.val.cmp(&key) {
            cmp::Ordering::Less => {
                Self::my_delete_node(&mut root_node.right, key);
            }
            cmp::Ordering::Greater => {
                Self::my_delete_node(&mut root_node.left, key);
            }
            cmp::Ordering::Equal => {
                let right = mem::replace(&mut root_node.right, None);
                if root_node.left.is_none() {
                    drop(root_node);
                    *root = right;
                } else {
                    let left = mem::replace(&mut root_node.left, None);
                    drop(root_node);
                    *root = left;
                    let mut current_node = Some(root.as_ref().unwrap().clone());
                    loop {
                        let current_right = Self::get_right(&current_node);
                        if current_right.is_none() {
                            break;
                        }
                        current_node = current_right;
                    }
                    current_node.as_ref().unwrap().borrow_mut().right = right;
                }
            }
        }
    }

    pub fn get_right(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().unwrap().borrow().right.clone()
    }
}


struct Solution;


fn build_tree(nums: &Vec<Option<i32>>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if idx >= nums.len() {
        return None;
    }
    let ret = Rc::new(RefCell::new(TreeNode::new(nums[idx]?)));
    {
        let mut ret_mut = ret.borrow_mut();
        ret_mut.left = build_tree(nums, idx * 2 + 1);
        ret_mut.right = build_tree(nums, idx * 2 + 2);
    }
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
            if s.is_empty() {
                break;
            }
            if s == "null" {
                nums.push(None);
            } else {
                nums.push(Some(s.parse().unwrap()));
            }
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::delete_node(build_tree(&nums, 0), input.trim().parse().unwrap()));
    }
}