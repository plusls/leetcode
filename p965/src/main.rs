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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_unival_tree_val(root.as_ref().unwrap()) != -1
    }

    pub fn get_unival_tree_val(root: &Rc<RefCell<TreeNode>>) -> i32 {
        use std::cell::Ref;
        let root: Ref<TreeNode> = root.try_borrow().unwrap();
        let n = root.val;
        if let Some(left) = root.left.as_ref() {
            let a = Self::get_unival_tree_val(left);
            if a == -1 || a != n {
                return -1;
            }
        }
        if let Some(right) = root.right.as_ref() {
            let a = Self::get_unival_tree_val(right);
            if a == -1 || a != n {
                return -1;
            }
        }
        n
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

        println!("{:?}", Solution::is_unival_tree(build_tree(&nums, 0)));
    }
}