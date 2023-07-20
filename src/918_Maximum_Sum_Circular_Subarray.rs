// 918_Maximum_Sum_Circular_Subarray
// https://leetcode.cn/problems/maximum-sum-circular-subarray/description/

impl Solution {
    fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut curr_max = 0;
        let mut curr_min = 0;

        for num in nums {
            curr_max = (curr_max + num).max(num);
            max_sum = max_sum.max(curr_max);
            curr_min = (curr_min + num).min(num);
            min_sum = min_sum.min(curr_min);
            total += num;
        }
        if max_sum > 0 {
            *[max_sum, total - min_sum].iter().max().unwrap()
        } else {
            max_sum
        }
    }      
}