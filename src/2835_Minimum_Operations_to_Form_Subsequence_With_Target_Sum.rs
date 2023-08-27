// 2835_Minimum_Operations_to_Form_Subsequence_With_Target_Sum
// https://leetcode.cn/problems/minimum-operations-to-form-subsequence-with-target-sum/description/

use std::collections::HashMap;

// Rust 需要处理整数溢出的情况
impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        if sum < target as i64 {
            return -1;
        }

        let mut count = HashMap::new();
        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut operations: i64 = 0;
        let mut total_sum: i64 = 0;

        for i in 0..31 {
            total_sum += *count.get(&(1 << i)).unwrap_or(&0) as i64 * (1 << i) as i64;

            if (target >> i & 1) == 0 {
                continue;
            }

            total_sum -= (1 << i) as i64;

            if total_sum >= 0 {
                continue;
            }

            for j in (i + 1)..31 {
                if let Some(cnt) = count.get_mut(&(1 << j)) {
                    if *cnt > 0 {
                        operations += j as i64 - i as i64;
                        *cnt -= 1;
                        total_sum += (1 << j) as i64;
                        break;
                    }
                }
            }
        }

        operations as i32
    }
}

