impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        for i in 0..num_rows as usize {
            ret.push(Vec::new());
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    ret[i].push(1);
                } else {
                    let v = ret[i - 1][j - 1] + ret[i - 1][j];
                    ret[i].push(v);
                }
            }
        }
        ret
    }
}

struct Solution();


fn main() {
    use std::io;

    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        println!("{:?}", Solution::generate(input.trim().parse().unwrap()));
    }
}
