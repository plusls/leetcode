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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).1
    }

    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        use std::cmp::min;
        if node.is_none() {
            return (i32::MAX / 2, 0, 0);
        }
        let node = node.as_ref().unwrap().borrow();
        let (la, lb, lc) = Self::dfs(&node.left);
        let (ra, rb, rc) = Self::dfs(&node.right);
        let a = lc + rc + 1;
        (a, min(a, min(la + rb, lb + ra)), min(a, lb + rb))
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

        println!("{:?}", Solution::min_camera_cover(build_tree(&nums, 0)));
    }
}