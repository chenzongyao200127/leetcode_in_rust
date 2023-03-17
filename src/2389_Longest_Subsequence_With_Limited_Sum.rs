// 2389_Longest_Subsequence_With_Limited_Sum
// https://leetcode.cn/problems/longest-subsequence-with-limited-sum/

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut pre = vec![];
        pre.push(0);
        let mut pre_sum = 0;
        for n in nums {
            pre_sum += n;
            pre.push(pre_sum);
        }
        
        let mut ans = vec![];
        for i in 0..queries.len() {
            let mut idx = 0;
            while idx < pre.len() && pre[idx as usize] <= queries[i] {
                idx += 1;
            }
            ans.push(idx as i32 - 1);
        }

        ans
    }
}


impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums;
        sorted_nums.sort();
        let n = sorted_nums.len();
        for i in 1 .. n {
            sorted_nums[i] += sorted_nums[i - 1];
        }

        queries.iter().map(|&query| {
            let mut left = 0;
            let mut right = n as i32 - 1;
            while left <= right {
                let mid = (left + right) / 2;
                if sorted_nums[mid as usize] <= query {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            left
        }).collect::<Vec<_>>()
    }
}