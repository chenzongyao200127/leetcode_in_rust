// 523. Continuous Subarray Sum
// https://leetcode.cn/problems/continuous-subarray-sum/

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut pre_sum = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        for i in 0..nums.len() {
            pre_sum = (nums[i] + pre_sum) % k ;
            if let Some(idx) = map.get(&(pre_sum)) {
                if (i as i32-idx) >= 2 {
                    return true;
                }
            } else {
                map.insert(pre_sum, i as i32);
            }
        }
        false
    }
}


use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut records = HashMap::new();
        records.insert(0, -1);
        let mut sum = 0;
        for (j, num) in nums.into_iter().enumerate() {
            sum = (sum + num) % k;
            if let Some(&i) = records.get(&sum) {
                if j as i32 - i > 1 {
                    return true;
                }
            } else {
                records.insert(sum, j as i32);
            }
        }
        false
    }
}

// class Solution {
//     public boolean checkSubarraySum(int[] nums, int k) {
//         int n = nums.length;
//         int[] sum = new int[n + 1];
//         for (int i = 1; i <= n; i++) sum[i] = sum[i - 1] + nums[i - 1];
//         Set<Integer> set = new HashSet<>();
//         for (int i = 2; i <= n; i++) {
//             set.add(sum[i - 2] % k);
//             if (set.contains(sum[i] % k)) return true;
//         }
//         return false;
//     }
// }

// 作者：AC_OIer
// 链接：https://leetcode.cn/problems/continuous-subarray-sum/solution/gong-shui-san-xie-tuo-zhan-wei-qiu-fang-1juse/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。