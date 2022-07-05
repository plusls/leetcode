use std::collections::BTreeMap;

struct MyCalendar {
    calendar: BTreeMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            calendar: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        // println!("self.calendar: {:?}", self.calendar);
        let it = self.calendar.range_mut(..end).next_back();
        if let Some((_k, v)) = it {
            // println!("k:{}, v:{}, start:{}, end:{}", _k, v, start, end);
            match (*v).cmp(&start) {
                std::cmp::Ordering::Equal => {
                    *v = end;
                    return true;
                }
                std::cmp::Ordering::Greater => {
                    return false;
                }
                _ => {}
            }
        }
        self.calendar.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

fn main() {
    use std::io;
    loop {
        let mut my_calendar_three = MyCalendar::new();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim().replace(' ', "");
        let mut input_nums: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            input_nums.push(Vec::new());
            let l = input_nums.len();
            for column in row.split(',') {
                input_nums[l - 1].push(column.parse().unwrap());
            }
        }
        let mut results: Vec<bool> = Vec::new();
        println!("new");
        for input_num in input_nums {
            println!("book [{}, {})", input_num[0], input_num[1]);
            let ret = my_calendar_three.book(input_num[0], input_num[1]);
            println!("book [{}, {}) -> {}", input_num[0], input_num[1], ret);

            results.push(ret);
        }

        println!("{:?}", results);
    }
}