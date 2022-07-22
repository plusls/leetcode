use std::io;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current_node = &mut dummy;
        for _ in 1..left {
            current_node = current_node.next.as_mut().unwrap();
        }

        if let Some(mut reversed_node) = current_node.next.take() {
            let mut next_node = reversed_node.next.take();
            for _ in left..right {
                let mut next_node_var = next_node.unwrap();
                next_node = next_node_var.next.replace(reversed_node);
                reversed_node = next_node_var;
            }
            current_node.next = Some(reversed_node);
            for _ in left..=right {
                current_node = current_node.next.as_mut().unwrap();
            }
            current_node.next = next_node;
        }


        dummy.next
    }
}

struct Solution();

fn read_list_node() -> Option<Box<ListNode>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut l = None;
    let mut prev_l: Option<&mut ListNode> = None;
    for n in input.split_whitespace() {
        let current_l = Box::new(ListNode { val: n.parse().unwrap(), next: None });
        if prev_l == None {
            l = Some(current_l);
            prev_l = Some(l.as_mut().unwrap());
        } else {
            let prev_node = prev_l.unwrap();
            prev_node.next = Some(current_l);
            prev_l = Some(prev_node.next.as_mut().unwrap());
        }
    }
    l
}

fn main() {
    loop {
        let l1 = read_list_node();
        if l1 == None {
            break;
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let left: i32 = input.trim().parse().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let right: i32 = input.trim().parse().unwrap();

        let mut result = Solution::reverse_between(l1, left, right);
        let mut first = true;
        while result != None {
            let current_node = result.unwrap();
            if first {
                print!("{}", current_node.val);
                first = false;
            } else {
                print!(" {}", current_node.val);
            }
            result = current_node.next;
        }
    }
}
