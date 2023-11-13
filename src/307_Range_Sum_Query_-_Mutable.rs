// 07_Range_Sum_Query_-_Mutable
// https://leetcode.cn/problems/range-sum-query-mutable/


struct NumArray {
    nums: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        NumArray {
            nums,
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        if index < self.nums.len() as i32 {
            self.nums[index as usize] = val;
        } else {
            // Handle error: index out of bounds
            // You can also choose to panic or return a Result type here
            println!("Index out of bounds");
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left > right || right >= self.nums.len() as i32 {
            // Handle error: invalid range
            // You can also choose to panic or return a Result type here
            println!("Invalid range");
            0
        } else {
            self.nums[left as usize ..=right as usize].iter().sum()
        }
    }
}

// https://leetcode.cn/problems/range-sum-query-mutable/
struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = vec![0; nums.len() + 1];
        for (i, &x) in nums.iter().enumerate() {
            let j = i + 1;
            tree[j] += x;
            let nxt = j + (j as i32 & -(j as i32)) as usize; // 下一个关键区间的右端点
            if nxt <= nums.len() {
                tree[nxt] += tree[j];
            }
        }
        Self { nums, tree }
    }

    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let delta = val - self.nums[index];
        self.nums[index] = val;
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += (i as i32 & -(i as i32)) as usize;
        }
    }

    fn prefix_sum(&self, i: i32) -> i32 {
        let mut s = 0;
        let mut i = i as usize;
        while i > 0 {
            s += self.tree[i];
            i &= i - 1; // i -= i & -i 的另一种写法
        }
        s
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum(right + 1) - self.prefix_sum(left)
    }
}
/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */