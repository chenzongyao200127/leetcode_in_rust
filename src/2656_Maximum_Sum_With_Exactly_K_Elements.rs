// 2656_Maximum_Sum_With_Exactly_K_Elements
// https://leetcode.cn/problems/maximum-sum-with-exactly-k-elements/description/

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let m = *nums.iter().max().unwrap();
        (m * 2 + k - 1) * k / 2
    }
}
