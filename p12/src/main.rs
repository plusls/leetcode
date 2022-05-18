use std::io;

const ROMAN_DATA: [[&str; 10]; 4] = [["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ["", "M", "MM", "MMM", "", "", "", "", "", ""]
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;

        let mut i = 0;
        let mut ret = String::new();
        while num > 0 {
            ret = [ROMAN_DATA[i][num as usize % 10].to_string(), ret].concat();
            num /= 10;
            i += 1;
        }
        ret
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
        println!("{}", Solution::int_to_roman(input.parse().unwrap()));
    }
}
