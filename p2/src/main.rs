use std::io;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    fn reverse_list_node(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node_option = l;
        let mut l_reverse = None;
        while current_node_option != None {
            let current_node = current_node_option.unwrap();
            l_reverse = Some(Box::new(ListNode { val: current_node.val, next: l_reverse }));
            current_node_option = current_node.next;
        }
        l_reverse
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_l1 = l1;
        let mut current_l2 = l2;

        let mut ret = None;
        let mut tmp = 0;
        loop {
            let mut val = current_l1.as_ref().map_or(0, |l_node| l_node.val) +
                current_l2.as_ref().map_or(0, |l_node| l_node.val) + tmp;
            tmp = val / 10;
            val %= 10;
            ret = Some(Box::new(ListNode {
                val,
                next: ret,
            }));

            current_l1 = current_l1.and_then(|l_node| l_node.next);
            current_l2 = current_l2.and_then(|l_node| l_node.next);
            if current_l1 == None && current_l2 == None {
                break;
            }
        }
        if tmp > 0 {
            ret = Some(Box::new(ListNode { val: tmp, next: ret }));
        }
        Solution::reverse_list_node(ret)
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
        let l2 = read_list_node();
        if l1 == None || l2 == None {
            break;
        }
        let mut result = Solution::add_two_numbers(l1, l2);
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
        println!();
    }
}
