use std::io;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut result: HashSet<String> = HashSet::new();
        for email in emails {
            let mut find_at = false;
            let mut ignore_until_at = false;
            let mut new_email = String::new();
            for ch in email.chars() {
                match ch {
                    '+' => {
                        ignore_until_at = true;
                    }
                    '.' => {
                        if !find_at {
                            continue;
                        }
                    }
                    '@' => {
                        find_at = true;
                    }
                    _ => {}
                }
                if !ignore_until_at || find_at {
                    new_email.push(ch);
                }
            }
            result.insert(new_email);
        }
        result.len() as i32
    }
}

struct Solution;


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let mut emails: Vec<String> = Vec::new();
        for n in input.trim().replace('[', "").replace(']', "").split("\",\"") {
            emails.push(n.trim().replace('"', ""));
        }

        println!("{:?}", Solution::num_unique_emails(emails));
    }
}
