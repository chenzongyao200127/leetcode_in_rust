// 643_Maximum_Average_Subarray_I
// https://leetcode.cn/problems/maximum-average-subarray-i/

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut tmp_sum = 0;
        for i in 0..k as usize {
            tmp_sum += nums[i];
        }                           // let mut tmp_sum = nums.iter().take(k as usize).sum::<i32>();

        let mut ans = tmp_sum as f64 / k as f64;
        for i in 0..nums.len() - k as usize {
            tmp_sum = tmp_sum - nums[i] + nums[i + k as usize];
            ans = ans.max(tmp_sum as f64 / k as f64);
        }

        ans
    }
}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut ans = nums.iter().take(k as usize).sum::<i32>();
        let k = k as usize;

        let mut cur_max = ans;
        for i in k..nums.len(){
            cur_max += (nums[i] - nums[i - k]) as i32;
            ans = ans.max(cur_max);
        }
        ans as f64 / k as f64
    }
}