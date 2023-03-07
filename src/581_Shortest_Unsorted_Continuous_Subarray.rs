// 581_Shortest_Unsorted_Continuous_Subarray
// https://leetcode.cn/problems/shortest-unsorted-continuous-subarray/

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut order_nums = nums.clone();
        let nums: Vec<_> = nums.iter().enumerate().collect();
        let mut ans = vec![];
        order_nums.sort_unstable();
        for i in 0..nums.len() {
            if *nums[i].1 != order_nums[i] {
                ans.push(nums[i].clone());
            }
        }

        if ans.is_empty() {
            return 0;
        } else {
            ans.sort_unstable_by(|a, b| (a.0).cmp(&(b.0)));
            (ans[ans.len()-1].0 - ans[0].0) as i32 + 1
        }
    }
}


impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut max_a = i32::MIN;
        let mut min_c = i32::MAX;
        let mut left = usize::MAX;
        let mut right = usize::MAX;

        for i in 0..len {
            if nums[i] < max_a {
                left = i;
            } else {
                max_a = nums[i];
            }

            if nums[len - 1 - i] > min_c {
                right = len - 1 - i;
            } else {
                min_c = nums[len - 1 - i];
            }
        }

        if right == usize::MAX {
            0
        } else if right < left {
            (left - right + 1) as i32
        } else {
            (right - left + 1) as i32
        }
    }
}