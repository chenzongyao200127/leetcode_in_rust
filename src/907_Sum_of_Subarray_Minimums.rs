// 907_Sum_of_Subarray_Minimums
// https://leetcode.cn/problems/sum-of-subarray-minimums/description/


impl Solution {
    fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mod_val = 1_000_000_007;
        let mut dp = vec![0; len];
        let mut stack: Vec<usize> = vec![];
    
        dp[0] = arr[0];
        stack.push(0);
    
        for i in 1..len {
            while let Some(&j) = stack.last() {
                if arr[j] > arr[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
    
            let &j = stack.last().unwrap_or(&i);
            if j == i {
                dp[i] = arr[i] * (i+1) as i32;
            } else {
                dp[i] = (dp[j] + arr[i] * (i-j) as i32) % mod_val;
            }
    
            stack.push(i);
        }
    
        dp.iter().fold(0, |sum, &val| (sum + val) % mod_val)
    }    
}