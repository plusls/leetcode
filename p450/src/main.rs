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
        use std::{cmp, mem};
        let new_root = Some(Rc::new(RefCell::new(TreeNode::new(i32::MAX))));
        new_root.as_ref().unwrap().borrow_mut().left = root;
        let mut current_root = new_root.as_ref().unwrap().borrow().left.clone();
        let mut prev_root = new_root.clone();
        let mut choose_left = true;
        // 寻找目标节点
        while current_root.is_some() {
            let current_root_node = current_root.as_ref().unwrap();
            let mut current_root_node = current_root_node.borrow_mut();
            match current_root_node.val.cmp(&key) {
                cmp::Ordering::Less => {
                    let tmp = current_root_node.right.clone();
                    drop(current_root_node);
                    prev_root = current_root;
                    current_root = tmp;
                    choose_left = false;
                }
                cmp::Ordering::Greater => {
                    let tmp = current_root_node.left.clone();
                    drop(current_root_node);
                    prev_root = current_root;
                    current_root = tmp;
                    choose_left = true;
                }
                cmp::Ordering::Equal => {
                    // 已经找到目标节点
                    let right = mem::replace(&mut current_root_node.right, None);
                    if current_root_node.left.is_none() {
                        // 如果目标节点左儿子是空，则直接用右儿子替换目标节点
                        Self::update_child(&prev_root, right, choose_left);
                    } else {
                        let left = mem::replace(&mut current_root_node.left, None);
                        drop(current_root_node);
                        // 用左儿子替换目标节点
                        Self::update_child(&prev_root, left.clone(), choose_left);
                        current_root = left;
                        // 寻找到左边儿子的最右的叶子节点，并将右儿子插在右边
                        loop {
                            // println!("current_root: {}", current_root.as_ref().unwrap().borrow().val);
                            let current_right = current_root.as_ref().unwrap().borrow().right.clone();
                            if current_right.is_none() {
                                break;
                            }
                            current_root = current_right;
                        }
                        current_root.as_ref().unwrap().borrow_mut().right = right;
                    }
                    break;
                }
            }
        }
        new_root.unwrap().borrow().left.clone()
    }

    pub fn update_child(root: &Option<Rc<RefCell<TreeNode>>>, child: Option<Rc<RefCell<TreeNode>>>, choose_left: bool) {
        if choose_left {
            root.as_ref().unwrap().borrow_mut().left = child;
        } else {
            root.as_ref().unwrap().borrow_mut().right = child;
        }
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