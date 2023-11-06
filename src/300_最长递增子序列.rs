// 300_最长递增子序列
// https://leetcode.cn/problems/longest-increasing-subsequence/description/

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = vec![nums[0]];
    
        for &n in &nums[1..] {
            let mut l = 0;
            let mut r = tails.len(); // Attention
            while l < r {
                let mid = l + (r - l) / 2;
                if tails[mid] < n {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
    
            if l == tails.len() {
                tails.push(n)
            } else {
                tails[l] = n
            }
        }   
    
        tails.len() as i32    
    }
}