impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        piles.sort_unstable();
        let mut l = 0_i64;
        piles.iter().for_each(|pile| l += *pile as i64);
        l /= h as i64;
        if l == 0 {
            l = 1;
        }
        let mut l = l as i32;
        let mut r = piles[piles.len() - 1] + 1;
        // println!("piles:{:?}, h:{}, l:{}, r:{}", piles, h, l, r);
        loop {
            assert!(l < r);
            let mid = (l + r - 1) / 2;
            let current_time = Self::calc_time(&piles, mid);
            if current_time <= h {
                if mid + 1 == r {
                    return mid;
                }
                r = mid + 1;
            } else {
                l = mid + 1;
            }
            // println!("l:{}, r:{}", l, r);
        }
    }

    pub fn calc_time(piles: &[i32], speed: i32) -> i32 {
        let mut ret = 0;
        for pile in piles {
            ret += pile / speed + if pile % speed == 0 { 0 } else { 1 };
        }
        ret
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
        let mut piles: Vec<i32> = Vec::new();
        for s in input.split(',') {
            let s = s.trim().replace('[', "").replace(']', "");
            piles.push(s.parse().unwrap());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", Solution::min_eating_speed(piles, input.trim().parse().unwrap()));
    }
}