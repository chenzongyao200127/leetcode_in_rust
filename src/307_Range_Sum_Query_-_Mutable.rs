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


/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */