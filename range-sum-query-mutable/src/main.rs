struct NumArray {
    nums: Vec<i32>,
    tree_array: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn low_bits(n: i32) -> i32 {
        n & -n
    }

    fn get_sum(&self, mut n: i32) -> i32 {
        n += 1;
        let mut sum = 0;
        while n >= 1 {
            sum += self.tree_array[n as usize - 1];
            n -= Self::low_bits(n);
        }
        sum
    }

    fn add(&mut self, mut n: i32, val: i32) {
        // println!("add idx: {} val: {}", n, val);
        // println!("nums: {:?} tree_array: {:?}", self.nums, self.tree_array);
        n += 1;
        while n <= self.nums.len() as i32 {
            self.tree_array[n as usize - 1] += val;
            n += Self::low_bits(n);
        }
    }


    fn new(nums: Vec<i32>) -> Self {
        let mut tree_array = vec![0; nums.len()];
        for (i, num) in nums.iter().enumerate() {
            tree_array[i] += num;
            let j = i as i32 + 1 + Self::low_bits(i as i32 + 1);
            if j <= tree_array.len() as i32 {
                tree_array[j as usize - 1] += tree_array[i];
            }
        }
        // println!("new nums: {:?} tree_array: {:?}", nums, tree_array);
        Self {
            nums,
            tree_array,
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        let diff = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        self.add(index, diff);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = self.get_sum(right);
        sum -= if left == 0 {
            0
        } else {
            self.get_sum(left - 1)
        };
        // println!("sum: {} nums: {:?} tree_array: {:?}", sum, self.nums, self.tree_array);
        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
fn main() {
    // ["NumArray","update","update","update","sumRange","update","sumRange","update","sumRange","sumRange","update"]
    // [[[7,2,7,2,0]],[4,6],[0,2],[0,9],[4,4],[3,8],[0,4],[4,1],[0,3],[0,4],[0,4]]
    let mut obj = NumArray::new(vec![7, 2, 7, 2, 0]);
    obj.update(4, 6);
    obj.update(0, 2);
    obj.update(0, 9);
    obj.sum_range(4, 4);
    obj.update(3, 8);
    obj.sum_range(0, 4);
    obj.update(4, 1);
    obj.sum_range(0, 3);
    obj.sum_range(0, 4);
    obj.update(0, 4);
}