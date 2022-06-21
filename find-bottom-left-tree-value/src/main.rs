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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root.unwrap(), 0).0
    }
    pub fn dfs(root: &Rc<RefCell<TreeNode>>, depth: usize) -> (i32, usize) {
        use std::cell::Ref;
        let root: Ref<TreeNode> = root.try_borrow().unwrap();
        let mut n = root.val;
        let mut ret_depth = depth;
        if let Some(left) = root.left.as_ref() {
            let (l_n, l_depth) = Self::dfs(left, depth + 1);
            n = l_n;
            ret_depth = l_depth;
        }
        if let Some(right) = root.right.as_ref() {
            let (r_n, r_depth) = Self::dfs(right, depth + 1);
            if r_depth > ret_depth {
                ret_depth = r_depth;
                n = r_n;
            }
        }
        // println!("val:{}, n:{}, depth:{}", root.val, n, depth);
        (n, ret_depth)
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

        println!("{:?}", Solution::find_bottom_left_value(build_tree(&nums, 0)));
    }
}