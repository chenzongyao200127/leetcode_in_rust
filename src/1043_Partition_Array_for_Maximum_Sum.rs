// 1043_Partition_Array_for_Maximum_Sum
// https://leetcode.cn/problems/partition-array-for-maximum-sum/

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0;arr.len() + 1];
        
        for i in 1..dp.len(){
            let mut max = 0;
            for j in (i.checked_sub(k).unwrap_or(0)..i).rev(){
                max = max.max(arr[j]);
                dp[i] = dp[i].max(dp[j] + max * (i - j) as i32);
            }
        }

        dp[dp.len()-1]
    }
}
