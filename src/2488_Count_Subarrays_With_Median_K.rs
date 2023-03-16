// 2488_Count_Subarrays_With_Median_K
// https://leetcode.cn/problems/count-subarrays-with-median-k/


/// 超时了...想了好久来着
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let max_len = (n-k).min(k) * 2 + 1;
        let mut k_idx = 0;
        for i in 0..nums.len() {
            if nums[i] == k {
                k_idx = i;
                break;
            }
        }

        let mut pre_sum = vec![0; nums.len()];
        let mut pre = 0;
        for i in 0..nums.len() {
            if nums[i] < k {
                pre -= 1;
            } else if nums[i] > k {
                pre += 1;
            }
            pre_sum[i] = pre;
        }
        // println!("{:?}", pre_sum);

        let mut ans = 0;
        for left in ((k_idx as i32 - (max_len - 1)).max(0)) as usize ..=k_idx {
            for right in k_idx..=((k_idx as i32 + (max_len - 1)).min(n-1)) as usize {
                if (right - left + 1) as i32 > max_len {
                    break;
                }
                // println!("{:?}", (left, right));
                if left > 0 {
                    let tmp = pre_sum[right] - pre_sum[left-1];
                    // println!("{:?}", (left, right, tmp));
                    if tmp == 0 || tmp == 1 {
                        ans += 1;
                    }
                } else {
                    let tmp = pre_sum[right];
                    if tmp == 0 || tmp == 1 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}


/// 示例代码
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k_index = nums.iter().position(|&x| x == k).unwrap();
        let mut i_count = HashMap::new();
        let mut j_count = HashMap::new();
        let mut cnt = 0;

        for i in (0..k_index).rev() {
            if nums[i] < k {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if let Some(&n) = i_count.get(&cnt) {
                i_count.insert(cnt, n + 1);
            } else {
                i_count.insert(cnt, 1);
            }
        }

        cnt = 0;
        for j in (k_index+1)..nums.len() {
            if nums[j] < k {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if let Some(&n) = j_count.get(&cnt) {
                j_count.insert(cnt, n + 1);
            } else {
                j_count.insert(cnt, 1);
            }
        }

        let mut res = 1;
        if let Some(&n) = i_count.get(&0) {
            res += n;
        }
        if let Some(&n) = i_count.get(&-1) {
            res += n;
        }
        if let Some(&n) = j_count.get(&0) {
            res += n;
        }
        if let Some(&n) = j_count.get(&-1) {
            res += n;
        }

        for (key, val) in i_count.iter() {
            if let Some(&n) = j_count.get(&(0 - key)){
                res += n * val;
            }
            if let Some(&n) = j_count.get(&(-1 - key)){
                res += n * val;
            }
        }
        res
    }
}


// class Solution:
//     def countSubarrays(self, nums: List[int], k: int) -> int:
//         pos, n = nums.index(k), len(nums)
//         cnt, x = [0] * (n * 2), n
//         cnt[x] = 1
//         for i in range(pos - 1, -1, -1):  # 从 pos-1 开始累加 x
//             x += 1 if nums[i] < k else -1
//             cnt[x] += 1

//         ans, x = cnt[n] + cnt[n - 1], n
//         for i in range(pos + 1, len(nums)):  # 从 pos+1 开始累加 x
//             x += 1 if nums[i] > k else -1
//             ans += cnt[x] + cnt[x - 1]
//         return ans