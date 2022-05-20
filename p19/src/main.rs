use std::io;

// 链表学习
// 折磨

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}


impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        assert!(head.is_some());
        let dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut first = dummy.as_ref();
        let mut second = dummy.as_ref();
        let mut idx = -1;
        while second.is_some() {
            if idx == n {
                first = first.unwrap().next.as_ref();
            } else {
                idx += 1;
            }
            second = second.unwrap().next.as_ref();
        }
        // println!("{:?} {:?}", first, second);
        let mut current_node;
        let first_ptr = first.unwrap() as *const Box<ListNode> as *mut Box<ListNode>;
        unsafe {
            current_node = first_ptr.as_mut().unwrap();
        }
        let target_node = current_node.next.take().unwrap().next;
        current_node.next = target_node;
        dummy.unwrap().next
    }
}

struct Solution;

fn to_list_node(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(nums[0])));
    if nums.len() == 1 {
        return head;
    }
    let mut current_node = head.as_mut().unwrap();
    for n in &nums[1..] {
        let list_node = Box::new(ListNode::new(*n));
        current_node.next = Some(list_node);
        current_node = current_node.next.as_mut().unwrap();
    }
    head
}

fn from_list_node(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    let mut current = head;
    while current.is_some() {
        let list_node = current.unwrap();
        nums.push(list_node.val);
        current = list_node.next;
    }
    nums
}

fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut nums: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.replace('[', "").replace(']', "");
            nums.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        println!("{:?}", from_list_node(Solution::remove_nth_from_end(to_list_node(nums), input.parse().unwrap())));
    }
}
