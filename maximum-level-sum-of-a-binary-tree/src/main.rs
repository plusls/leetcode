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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut depth = 0;
        let root = root.unwrap();

        let mut ret = 0;
        let mut max_sum = i32::MIN;

        let mut next_depth_node = root.clone();
        let mut need_update_next_depth_node = false;


        queue.push_back(root);
        let mut current_sum = i32::MIN;
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();
            if next_depth_node == current_node {
                need_update_next_depth_node = true;
                if current_sum > max_sum {
                    max_sum = current_sum;
                    ret = depth;
                }
                current_sum = 0;
                depth += 1;
            }
            current_sum += current_node.borrow().val;
            if let Some(left) = current_node.borrow_mut().left.take() {
                if need_update_next_depth_node {
                    next_depth_node = left.clone();
                    need_update_next_depth_node = false;
                }
                queue.push_back(left);
            }


            if let Some(right) = current_node.borrow_mut().right.take() {
                if need_update_next_depth_node {
                    next_depth_node = right.clone();
                    need_update_next_depth_node = false;
                }
                queue.push_back(right);
            }
            drop(current_node);
        }
        if current_sum > max_sum {
            ret = depth;
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
            if s == "null" {
                nums.push(None);
            } else {
                nums.push(Some(s.parse().unwrap()));
            }
        }

        println!("{:?}", Solution::max_level_sum(build_tree(&nums, 0)));
    }
}