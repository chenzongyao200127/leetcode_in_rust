// 2357. Make Array Zero by Subtracting Equal Amounts
// https://leetcode.cn/problems/make-array-zero-by-subtracting-equal-amounts/

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        while !nums.iter().all(|x| *x == 0) {
            let mut min = 100000;
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    continue;
                }
                min = min.min(nums[i]);
            }
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    continue;
                }
                nums[i] = nums[i] - min;
            }
            ans += 1;

        }

        ans
    }
}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        let mut pre = 0;
        for item in nums.into_iter() {
            if item == 0 {
                continue;
            }
            if item != pre {
                ans += 1;
            }
            pre = item
        }
        
        ans
    }
}


impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = [0; 101];
        let mut ans = 0;
        for num in nums {
            if num == 0 {
                continue;
            }
            if (cnt[num as usize] == 0) {
                ans += 1;
            }
            cnt[num as usize] += 1;
        }
        ans
    }
}
