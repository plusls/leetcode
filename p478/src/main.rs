struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

// extern crate rand;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self { radius, x_center, y_center }
    }

    fn rand_point(&self) -> Vec<f64> {
        // 看的题解，不会概率论
        use std::f64::consts::PI;
        loop {
            let r = rand::random::<f64>().sqrt() * self.radius;
            let theta: f64 = rand::random::<f64>() * 2_f64 * PI;
            let x = self.x_center + r * theta.cos();
            let y = self.y_center + r * theta.sin();
            if (x - self.x_center).powi(2) + (y - self.y_center).powi(2) <= self.radius.powi(2)
            {
                return vec![x, y];
            }
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */


fn main() {}
