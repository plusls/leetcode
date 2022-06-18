use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: HashMap<u8, Rc<RefCell<Node>>>,
    max_idx: usize,
}

#[derive(Debug)]
struct WordFilter {
    words: Rc<RefCell<Node>>,
    count: usize,
}


impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        // let mut ret = Self { words_tree: Box::new(TreeNode { next: HashMap::new(), max_idx: usize::MAX }) };
        let ret = Self {
            words: Rc::new(RefCell::new(
                Node {
                    next: HashMap::new(),
                    max_idx: usize::MAX,
                })),
            count: words.len(),
        };
        for (i, word) in words.iter().enumerate() {
            let new_word = word.clone() + "#" + &*word;
            for j in 0..word.len() + 1 {
                ret.insert(&new_word.as_bytes()[j..], i)
            }
        }
        ret
    }

    fn insert(&self, word: &[u8], idx: usize) {
        // println!("insert: {:?}", String::from_utf8(Vec::from(word)));
        let mut current_node = self.words.clone();
        for ch in word {
            match current_node.clone().borrow_mut().next.entry(*ch) {
                Entry::Occupied(mut occupied) => {
                    let occupied = occupied.get_mut();
                    occupied.borrow_mut().max_idx = idx;
                    current_node = occupied.clone();
                }
                Entry::Vacant(vacant) => {
                    current_node = vacant.insert(Rc::new(RefCell::new(
                        Node {
                            next: HashMap::new(),
                            max_idx: idx,
                        }))).clone();
                }
            }
            // println!("insert: {}, idx: {}", *ch as char, idx);
        }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let s = suffix + "#" + &*prefix;

        let mut current_node = self.words.clone();
        let mut ret = self.count as i32;
        for ch in s.as_bytes() {
            if let Some(next_node) = current_node.clone().borrow().next.get(ch) {
                ret = next_node.borrow().max_idx as i32;
                current_node = next_node.clone();
            } else {
                ret = -1;
                break;
            }
            // println!("search: {}, ret: {}", *ch as char, ret);
        }
        // println!("ret: {}", ret);
        ret
    }
}

fn main() {
    let word_filter = WordFilter::new(vec!["bbabaabaab".to_string(),
                                           "bbababbaab".to_string(),
                                           "apple".to_string(),
                                           "apple".to_string(),
                                           "bcd".to_string()]);
    word_filter.f("a".to_string(), "e".to_string());
    word_filter.f("apple".to_string(), "e".to_string());
    word_filter.f("b".to_string(), "c".to_string());
    word_filter.f("b".to_string(), "".to_string());
    word_filter.f("bbabaa".to_string(), "baab".to_string());
    word_filter.f("bbabaab".to_string(), "baab".to_string());
}