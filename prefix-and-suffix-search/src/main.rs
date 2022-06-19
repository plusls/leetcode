use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    next: HashMap<u8, Box<Node>>,
    max_idx: usize,
}

#[derive(Debug)]
struct WordFilter {
    words: Box<Node>,
}


impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        // let mut ret = Self { words_tree: Box::new(TreeNode { next: HashMap::new(), max_idx: usize::MAX }) };
        let mut ret = Self {
            words: Box::new(
                Node {
                    next: HashMap::new(),
                    max_idx: usize::MAX,
                }),
        };
        for (i, word) in words.iter().enumerate() {
            let new_word = word.clone() + "#" + &*word;
            for j in 0..word.len() + 1 {
                ret.insert(&new_word.as_bytes()[j..], i)
            }
        }
        ret
    }

    fn insert(&mut self, word: &[u8], idx: usize) {
        // println!("insert: {:?}", String::from_utf8(Vec::from(word)));
        word.iter().fold(&mut self.words, |node, &ch| {
            let next_node = node.next.entry(ch).or_insert_with(|| Box::new(Node {
                next: HashMap::new(),
                max_idx: usize::MAX,
            }));
            next_node.max_idx = idx;
            next_node
        });
        // println!("insert: {}, idx: {}", *ch as char, idx);
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let s = suffix + "#" + &*prefix;

        s.as_bytes().iter().try_fold(&self.words, |node, &ch| {
            match node.next.get(&ch) {
                Some(node) => Ok(node),
                None => Err(()),
            }
        }).map_or(-1, |node| {
            node.max_idx as i32
        })
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
    word_filter.f("".to_string(), "".to_string());
}