// 456_132_Pattern
// https://leetcode.cn/problems/132-pattern/


impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stk = vec![];
        let mut third = i32::MIN;
        for i in (0..nums.len()).rev() {
            if nums[i] < third {
                return true;
            }

            while !stk.is_empty() && nums[i] > stk[stk.len() - 1] {
                third = stk.pop().unwrap();
            }
            stk.push(nums[i]);
        }

        false
    }
}



use std::cmp::max;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut max_k = i32::MIN;
        for &num in nums.iter().rev() {
            // 判断是否能作为1
            if num < max_k {
                return true;
            }
            // 是否可以做3
            while !stack.is_empty() && *stack.last().unwrap() < num {
                max_k = stack.pop().unwrap();
            }
            stack.push(num);
        }

        return false;
    }
}

