#[derive(Debug)]
struct Solution {
    rects: Vec<Vec<i32>>,
    point_nums: Vec<i32>,
}

// extern crate rand;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut point_nums: Vec<i32> = Vec::new();
        for rect in &rects {
            let x = rect[2] - rect[0] + 1;
            let y = rect[3] - rect[1] + 1;
            point_nums.push(if point_nums.is_empty() { 0 } else { point_nums[point_nums.len() - 1] } + x * y);
        }
        Self {
            rects,
            point_nums,
        }
    }

    fn pick(&self) -> Vec<i32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut l = 0;
        let mut r = self.point_nums.len();
        let mut n = rng.gen_range(0, self.point_nums[r - 1]);
        let idx;
        loop {
            let mid = (l + r - 1) / 2;
            if self.point_nums[mid] <= n {
                l = mid + 1;
            } else {
                if r == mid + 1 {
                    idx = mid;
                    break;
                }
                r = mid + 1;
            }
        }
        let rect = &self.rects[idx];
        if idx != 0 {
            n -= self.point_nums[idx - 1];
        }
        let x = rect[0] + n % (rect[2] - rect[0] + 1);
        let y = rect[1] + n / (rect[2] - rect[0] + 1);
        vec![x, y]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */


fn main() {
    let input = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
    let input = vec![vec![-58953616, -40483558, -58953446, -40482555],
                     vec![76369640, 94978791, 76371036, 94979394],
                     vec![80970826, -37466957, 80971657, -37466388],
                     vec![-79821573, -4177978, -79820536, -4177925]];
    let solution = Solution::new(input);
    for _ in 0..1000 {
        println!("{:?}", solution.pick());
    }
}
