use std::io;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left_pos: i32,
    right_pos: i32,
    lazy: bool,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

trait SegmentTree {
    fn update(&mut self, val: i32, left_pos: i32, right_pos: i32);
    fn query(&self, left_pos: i32, right_pos: i32) -> i32;
}

impl SegmentTree for TreeNode {
    fn update(&mut self, val: i32, left_pos: i32, right_pos: i32) {
        use std::cmp::{max, min};
        // println!("update val:{}, left_pos:{}, right_pos:{} self.l:{}, self.r:{}", val, left_pos, right_pos, self.left_pos, self.right_pos);
        if left_pos == self.left_pos && right_pos == self.right_pos {
            self.val = val;
            self.lazy = true;
            return;
        }
        let mid = (self.left_pos + self.right_pos) / 2;
        assert!(left_pos < mid || right_pos >= mid);
        if self.lazy {
            if let Some(left) = self.left.as_mut() {
                left.val = self.val;
                left.lazy = true;
            }
            if let Some(right) = self.right.as_mut() {
                right.val = self.val;
                right.lazy = true;
            }
        }
        if self.left.is_none() {
            self.left = Some(Box::new(TreeNode {
                val: if self.lazy { self.val } else { 0 },
                left_pos: self.left_pos,
                right_pos: mid,
                lazy: true,
                left: None,
                right: None,
            }));
        }
        if self.right.is_none() {
            self.right = Some(Box::new(TreeNode {
                val: if self.lazy { self.val } else { 0 },
                left_pos: mid,
                right_pos: self.right_pos,
                lazy: true,
                left: None,
                right: None,
            }));
        }
        if left_pos < mid {
            self.left.as_mut().unwrap().update(val, left_pos, min(mid, right_pos));
        }
        if right_pos > mid {
            self.right.as_mut().unwrap().update(val, max(mid, left_pos), right_pos);
        }
        self.lazy = false;
        self.val = max(self.left.as_ref().unwrap().val, self.right.as_ref().unwrap().val);
    }

    fn query(&self, left_pos: i32, right_pos: i32) -> i32 {
        use std::cmp::{max, min};
        if self.lazy || (left_pos == self.left_pos && right_pos == self.right_pos) {
            return self.val;
        }
        assert!(self.left.is_some() && self.right.is_some());
        let mid = (self.left_pos + self.right_pos) / 2;
        let mut left: Option<i32> = None;
        let mut right: Option<i32> = None;
        assert!(left_pos < mid || right_pos >= mid);
        if left_pos < mid {
            left = Some(self.left.as_ref().unwrap().query(left_pos, min(mid, right_pos)));
        }
        if right_pos > mid {
            right = Some(self.right.as_ref().unwrap().query(max(mid, left_pos), right_pos));
        }
        if let Some(left) = left {
            if let Some(right) = right {
                max(left, right)
            } else {
                left
            }
        } else {
            right.unwrap()
        }
    }
}

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut l = positions[0][0];
        let mut r = l + positions[0][1];
        for position in &positions[1..] {
            let left = position[0];
            let right = left + position[1];
            if left < l {
                l = left;
            }
            if right > r {
                r = right;
            }
        }
        let mut root = TreeNode { val: 0, left_pos: l, right_pos: r, lazy: true, left: None, right: None };
        let mut ret = Vec::new();
        for n in positions {
            let left = n[0];
            let right = left + n[1];
            let max_height = root.query(left, right);
            // println!("update height:{} new_height:{} left:{} right:{}", n[1], n[1] + max_height, left, right);

            root.update(n[1] + max_height, left, right);
            ret.push(root.val);
            // println!("root: {:?}", root);
        }
        ret
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let input = input.replace(' ', "");
        let mut positions: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            positions.push(Vec::new());
            let l = positions.len();
            for column in row.split(',') {
                positions[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::falling_squares(positions));
    }
}
