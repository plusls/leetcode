#[derive(Debug)]
struct TreeNode {
    start: i32,
    end: i32,
    val: Option<bool>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

trait SegmentTree {
    fn update(&mut self, start: i32, end: i32, val: bool);
    fn query(&self, start: i32, end: i32) -> bool;
}

impl SegmentTree for TreeNode {
    fn update(&mut self, start: i32, end: i32, val: bool) {
        use std::cmp::{max, min};
        let mid = (self.start + self.end) / 2;
        assert!(start < mid || end >= mid);

        if let Some(v) = self.val {
            if v == val {
                return;
            } else {
                self.left.get_or_insert(Box::new(TreeNode {
                    start: self.start,
                    end: mid,
                    val: None,
                    left: None,
                    right: None,
                })).val = self.val;

                self.right.get_or_insert(Box::new(TreeNode {
                    start: mid,
                    end: self.end,
                    val: None,
                    left: None,
                    right: None,
                })).val = self.val;
                self.val = None;
            }
        }
        // println!("update val:{}, start:{}, end:{} self.start:{}, self.end:{}", val, start, end, self.start, self.end);
        if start == self.start && end == self.end {
            self.val = Some(val);
            // println!("set [{}, {}) => {} self.lazy:{} val:{}", self.start, self.end, self.val, self.lazy, val);
            return;
        }

        if start < mid {
            self.left.as_mut().unwrap().update(start, min(mid, end), val);
        }

        if end > mid {
            self.right.as_mut().unwrap().update(max(mid, start), end, val);
        }
    }

    fn query(&self, start: i32, end: i32) -> bool {
        use std::cmp::{max, min};

        if let Some(v) = self.val {
            return v;
        }
        let mid = (self.start + self.end) / 2;


        let (mut left, mut right) = (true, true);
        if start < mid {
            left = self.left.as_ref().unwrap().query(start, min(mid, end));
        }

        if end > mid {
            right = self.right.as_ref().unwrap().query(max(mid, start), end);
        }
        left && right
    }
}


impl RangeModule {
    fn new() -> Self {
        Self {
            tree_node: TreeNode { start: 1, end: 1000000009, val: Some(false), left: None, right: None },
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        self.tree_node.update(left, right, true);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.tree_node.query(left, right)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        self.tree_node.update(left, right, false);
    }
}

struct RangeModule {
    tree_node: TreeNode,
}


fn main() {
    use std::io;
}