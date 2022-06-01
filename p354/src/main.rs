use std::io;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        let mut envelopes = envelopes;
        envelopes.sort_unstable_by(|envelope0, envelope1| {
            let envelope0_0 = envelope0[0];
            let envelope0_1 = envelope0[1];
            let envelope1_0 = envelope1[0];
            let envelope1_1 = envelope1[1];
            match envelope0_0.cmp(&envelope1_0) {
                cmp::Ordering::Equal => {
                    envelope1_1.cmp(&envelope0_1)
                }
                ordering => {
                    ordering
                }
            }
        });
        // println!("envelopes: {:?}", envelopes);
        let mut d = vec![0; envelopes.len()];
        let mut ret = 0;
        d[ret] = envelopes[0][1];
        for t in envelopes {
            let n = t[1];
            match n.cmp(&d[ret]) {
                cmp::Ordering::Greater => {
                    ret += 1;
                    d[ret] = n;
                }
                cmp::Ordering::Less => {
                    let mut l = 0;
                    let mut r = ret + 1;
                    loop {
                        let mid = (l + r) / 2;
                        match n.cmp(&d[mid]) {
                            cmp::Ordering::Greater => {
                                l = mid + 1;
                            }
                            cmp::Ordering::Equal => {
                                break;
                            }
                            cmp::Ordering::Less => {
                                r = mid;
                            }
                        }

                        if l == r {
                            d[l] = n;
                            break;
                        }
                    }
                }
                _ => {}
            }
            // println!("n:{}, d: {:?}", n, d);
        }
        ret as i32 + 1
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

        let mut envelopes: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            envelopes.push(Vec::new());
            let l = envelopes.len();
            for column in row.split(',') {
                envelopes[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::max_envelopes(envelopes));
    }
}
