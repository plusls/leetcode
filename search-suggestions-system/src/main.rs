use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    next: HashMap<u8, Box<Node>>,
    idxs_vec: Vec<usize>,
}


impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut words = Box::new(Node { next: HashMap::new(), idxs_vec: Vec::new() });
        for (i, word) in products.iter().enumerate() {
            Self::insert(&mut words, word.as_bytes(), i);
        }
        let mut ret = Vec::new();
        search_word.as_bytes().iter().fold(Some(&mut words), |node, &ch| {
            if node.is_none() {
                ret.push(Vec::new());
                return None;
            }

            match node.unwrap().next.get_mut(&ch) {
                Some(next_node) => {
                    next_node.idxs_vec.sort_by(|&a, &b| products[a].cmp(&products[b]));
                    let mut current = Vec::new();
                    for (i, &idx) in next_node.idxs_vec.iter().enumerate() {
                        if i >= 3 {
                            break;
                        }
                        current.push(products[idx].clone());
                    }
                    ret.push(current);
                    Some(next_node)
                }
                None => {
                    ret.push(Vec::new());
                    None
                }
            }
        });

        ret
    }
    fn insert(words: &mut Box<Node>, word: &[u8], idx: usize) {
        // println!("insert: {:?}", String::from_utf8(Vec::from(word)));
        word.iter().fold(words, |node, &ch| {
            let next_node = node.next.entry(ch).or_insert_with(|| Box::new(Node {
                next: HashMap::new(),
                idxs_vec: Vec::new(),
            }));
            next_node.idxs_vec.push(idx);
            next_node
        });
        // println!("insert: {}, idx: {}", *ch as char, idx);
    }
}

struct Solution;


fn main() {
    use std::io;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let mut words: Vec<String> = Vec::new();
        for n in input.split("\",\"") {
            let n = n.trim().replace("[\"", "").replace("\"]", "");
            words.push(n);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{:?}", Solution::suggested_products(words, input.replace('"', "").trim().to_string()));
    }
}
