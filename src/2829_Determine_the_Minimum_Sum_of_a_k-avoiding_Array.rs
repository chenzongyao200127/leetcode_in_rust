// 2829_Determine_the_Minimum_Sum_of_a_k-avoiding_Array
// https://leetcode.cn/problems/determine-the-minimum-sum-of-a-k-avoiding-array/description/?envType=daily-question&envId=2025-03-26
impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let m = n.min(k / 2);
        let sum_m = m * (m + 1) / 2; // Sum of first m natural numbers
        let sum_rest = (k..).take((n - m) as usize).sum::<i32>(); // Sum of the next (n - m) numbers starting from k
        sum_m + sum_rest
    }
}
