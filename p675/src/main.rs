use std::io;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

        for (i, row) in forest.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                heap.push(Reverse((*column, i, j)));
            }
        }
        let mut x = 0_usize;
        let mut y = 0_usize;
        let mut ret = 0;
        while !heap.is_empty() {
            let v = heap.pop().unwrap().0;
            if v.0 == 0 || v.0 == 1 {
                continue;
            }
            let r = Self::bfs(&forest, x, y, v.1, v.2);
            if r == -1 {
                ret = -1;
                break;
            }
            ret += r;
            x = v.1;
            y = v.2;
            //println!("{:?}", heap.pop().unwrap().0);
        }

        ret
    }

    pub fn bfs(forest: &[Vec<i32>], x0: usize, y0: usize, x1: usize, y1: usize) -> i32 {
        use std::collections::VecDeque;
        let m = forest.len();
        let n = forest[0].len();
        let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited = vec![false; m * n];
        deque.push_back((x0, y0, 0));
        while !deque.is_empty() {
            let (x, y, p_l) = deque.pop_front().unwrap();
            if visited[x * n + y] {
                continue;
            }
            visited[x * n + y] = true;
            if x == x1 && y == y1 {
                return p_l as i32;
            }
            if x >= 1 && forest[x - 1][y] != 0 {
                // if x - 1 == 12 && y == 12{
                //     println!("{}", visited[12*n+12]);
                // }
                deque.push_back((x - 1, y, p_l + 1));
            }
            if x + 1 < m && forest[x + 1][y] != 0 {
                // if x + 1 == 12 && y == 12{
                //     println!("{}", visited[12*n+12]);
                // }
                deque.push_back((x + 1, y, p_l + 1));
            }
            if y >= 1 && forest[x][y - 1] != 0 {
                // if x == 12 && y-1 == 12{
                //     println!("{}", visited[12*n+12]);
                // }
                deque.push_back((x, y - 1, p_l + 1));
            }
            if y + 1 < n && forest[x][y + 1] != 0 {
                // if x == 12 && y+1 == 12{
                //     println!("{}", visited[12*n+12]);
                // }
                deque.push_back((x, y + 1, p_l + 1));
            }
        }
        -1
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

        let mut forest: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            forest.push(Vec::new());
            let l = forest.len();
            for column in row.split(',') {
                forest[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::cut_off_tree(forest));
    }
}
