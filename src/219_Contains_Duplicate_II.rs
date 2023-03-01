// 219. Contains Duplicate II
// https://leetcode.cn/problems/contains-duplicate-ii/

use std::collections::HashMap;

// 给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，
// 满足 nums[i] == nums[j] 且 abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false 。
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut min = i32::MAX;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            map.entry(nums[i]).and_modify(|idx| {
                min = min.min((i-*idx) as i32);
                *idx = i;
            }).or_insert(i);
            if min <= k {
                return true;
            }
        }
    
        min <= k
    }
}

// 方法二：滑动窗口
use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut s: HashSet<i32> = HashSet::new();
        for (i, val) in nums.iter().enumerate() {
            if s.contains(val) {
                return true;
            }

            s.insert(*val);
            if s.len() >= (k + 1) as usize {
                let offset = i - k as usize;
                s.remove(&nums[offset]);
            }
        }
        false
    }
}

// // 方法二：滑动窗口 Java
// class Solution {
//     public boolean containsNearbyDuplicate(int[] nums, int k) {
//         Set<Integer> set = new HashSet<Integer>();
//         int length = nums.length;
//         for (int i = 0; i < length; i++) {
//             if (i > k) {
//                 set.remove(nums[i - k - 1]);
//             }
//             if (!set.add(nums[i])) {
//                 return true;
//             }
//         }
//         return false;
//     }
// }