// 1574_Shortest_Subarray_to_be_Removed_to_Make_Array_Sorted
// https://leetcode.cn/problems/shortest-subarray-to-be-removed-to-make-array-sorted/

// 阿里笔试
/// 双指针
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut i = 0;
        let mut j = n - 1;
    
        while i < n - 1 && arr[i] <= arr[i + 1] {
            i += 1;
        }
        while j > 0 && arr[j-1] <= arr[j] {
            j -= 1;
        }
        if i >= j {
            return 0;
        }
    
        let mut ans = j.min(n-i-1);
        let mut right = j;
        for left in 0..i+1 {
            while right < n && arr[right] < arr[left] {
                right += 1;
            }
            ans = ans.min(right - left - 1);
        }
        
        ans as i32
    }
}

/// 错误思路：
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut dp = vec![1; arr.len()];
        dp[0] = 1;
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[j] <= arr[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        arr.len() as i32 - dp.iter().max().unwrap()
    }
}

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut dp = vec![0; arr.len()];
        dp[0] = 1;
        for i in 1..arr.len() {
            if arr[i] > arr[i-1] {
                dp[i] = dp[i-1] + 1;
            } else {
                for j in (0..i).rev() {
                    if arr[j] <= arr[i] {
                        dp[i] = dp[j] + 1;
                        break;
                    }
                }
            }
        }

        arr.len() as i32 - dp.iter().max().unwrap()
    }
}