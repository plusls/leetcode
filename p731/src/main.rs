use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    start: i32,
    end: i32,
    lazy: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

trait SegmentTree {
    fn update(&mut self, val: i32, start: i32, end: i32);
    fn query(&mut self, start: i32, end: i32) -> i32;
}

impl SegmentTree for TreeNode {
    fn update(&mut self, val: i32, start: i32, end: i32) {
        use std::cmp::{max, min};
        // println!("update val:{}, start:{}, end:{} self.start:{}, self.end:{}", val, start, end, self.start, self.end);
        if start == self.start && end == self.end {
            self.val += val;
            self.lazy += val;
            // println!("set [{}, {}) => {} self.lazy:{} val:{}", self.start, self.end, self.val, self.lazy, val);
            return;
        }
        let mid = (self.start + self.end) / 2;
        assert!(start < mid || end >= mid);

        if self.left.is_none() {
            assert!(self.right.is_none());
            self.left = Some(Box::new(TreeNode {
                val: self.val,
                start: self.start,
                end: mid,
                lazy: 0,
                left: None,
                right: None,
            }));
            self.right = Some(Box::new(TreeNode {
                val: self.val,
                start: mid,
                end: self.end,
                lazy: 0,
                left: None,
                right: None,
            }));
            self.lazy = 0;
        }
        let left_mut_ref = self.left.as_mut().unwrap();
        left_mut_ref.lazy += self.lazy;
        left_mut_ref.val += self.lazy;
        if start < mid {
            left_mut_ref.update(val, start, min(mid, end));
        }
        let right_mut_ref = self.right.as_mut().unwrap();

        right_mut_ref.lazy += self.lazy;
        right_mut_ref.val += self.lazy;
        if end > mid {
            right_mut_ref.update(val, max(mid, start), end);
        }
        self.lazy = 0;
        self.val = max(left_mut_ref.val, right_mut_ref.val);
    }

    fn query(&mut self, start: i32, end: i32) -> i32 {
        use std::cmp::{max, min};
        if self.left.is_none() || (start == self.start && end == self.end) {
            return self.val;
        }
        assert!(self.left.is_some() && self.right.is_some());
        let mid = (self.start + self.end) / 2;
        let mut left: Option<i32> = None;
        let mut right: Option<i32> = None;
        assert!(start < mid || end > mid);
        let left_mut_ref = self.left.as_mut().unwrap();
        left_mut_ref.lazy += self.lazy;
        left_mut_ref.val += self.lazy;
        if start < mid {
            left = Some(left_mut_ref.query(start, min(mid, end)));
        }
        let right_mut_ref = self.right.as_mut().unwrap();
        right_mut_ref.lazy += self.lazy;
        right_mut_ref.val += self.lazy;
        if end > mid {
            right = Some(right_mut_ref.query(max(mid, start), end));
        }
        self.lazy = 0;
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


struct MyCalendarTwo {
    tree_node: RefCell<TreeNode>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            tree_node: RefCell::new(TreeNode { val: 0, start: 0, end: 1000000001, lazy: 0, left: None, right: None })
        }
    }

    fn book(&self, start: i32, end: i32) -> bool {
        if self.tree_node.borrow_mut().query(start, end) < 2 {
            self.tree_node.borrow_mut().update(1, start, end);
            true
        } else {
            false
        }
    }
}

fn main() {
    use std::io;
    loop {
        let my_calendar_two = MyCalendarTwo::new();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim().replace(' ', "");
        let mut input_nums: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            input_nums.push(Vec::new());
            let l = input_nums.len();
            for column in row.split(',') {
                input_nums[l - 1].push(column.parse().unwrap());
            }
        }
        let mut results = Vec::new();
        println!("new");
        for input_num in input_nums {
            println!("book [{}, {})", input_num[0], input_num[1]);
            let ret = my_calendar_two.book(input_num[0], input_num[1]);
            println!("book [{}, {}) -> {}", input_num[0], input_num[1], ret);

            results.push(ret);
        }

        println!("{:?}", results);
    }
}