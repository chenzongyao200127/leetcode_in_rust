// 128. Longest Consecutive Sequence
// https://leetcode.cn/problems/longest-consecutive-sequence/
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut H: HashSet<i32> = HashSet::new();
        for i in nums {
            if let Some(_) = H.get(&i) {
                continue;
            }
            H.insert(i);
        }
        let mut nums: Vec<i32> = H.into_iter().collect();
        nums.sort_unstable();
        let mut ans = 0;
        let mut cnt = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] + 1 {
                cnt += 1
            } else {
                ans = ans.max(cnt);
                cnt = 1;
            }
        }
        ans = ans.max(cnt);
        ans
    }
}

// if let Some(_) = H.get(&i) {
//     return true
// }
// H.insert(i);

use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        for &num in nums.iter() {
            cache.insert(num, true);
        }
        let mut max_len = 0;
        for &k in cache.keys() {
            if cache.contains_key(&(k - 1)) {
                continue;
            }
            let mut cnt = 0;
            let mut k = k;
            while cache.contains_key(&k) {
                cnt += 1;
                k += 1;
            }
            max_len = max_len.max(cnt);
        }
        max_len
    }
}

// 解题思路1：哈希集合
// class Solution {
//     public int longestConsecutive(int[] nums) {
//         // 建立一个存储所有数的哈希表，同时起到去重功能
//         Set<Integer> set = new HashSet<>();
//         for (int num : nums) {
//             set.add(num);
//         }

//         int ans = 0;
//         // 遍历去重后的所有数字
//         for (int num : set) {
//             int cur = num;
//             // 只有当num-1不存在时，才开始向后遍历num+1，num+2，num+3......
//             if (!set.contains(cur - 1)) {
//                 while (set.contains(cur + 1)) {
//                     cur++;
//                 }
//             }
//             // [num, cur]之间是连续的，数字有cur - num + 1个
//             ans = Math.max(ans, cur - num + 1);
//         }
//         return ans;
//     }
// }
// 作者：yimeixiaobai
// 链接：https://leetcode.cn/problems/longest-consecutive-sequence/solution/xiao-bai-lang-ha-xi-ji-he-ha-xi-biao-don-j5a2/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let H: HashSet<i32> = HashSet::new();
        for i in nums {
            H.insert(i);
        }
        let mut ans = 0;
        let mut cnt = 1;
        for i in nums {
            if !H.contains(i-1) {
                while H.contains (i+1) {
                    cnt += 1;
                }
            }
            ans = ans.mas(cnt);
        }

        ans
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for i in nums.into_iter() {
            set.insert(i);
        }
        let mut ans = 0;
        for i in set.iter() {
            if !set.contains(&(i-1)) {
                let mut cnt = 1;
                while set.contains(&(i+cnt)) {
                    cnt += 1;
                }
                ans = ans.max(cnt);
            }
        }

        ans
    }
}


// class Solution {
//     public int longestConsecutive(int[] nums) {
//         // key表示num，value表示num最远到达的连续右边界
//         Map<Integer, Integer> map = new HashMap<>();
//         // 初始化每个num的右边界为自己
//         for (int num : nums) {
//             map.put(num, num);
//         }

//         int ans = 0;
//         for (int num : nums) {
//             if (!map.containsKey(num - 1)) {
//                 int right = map.get(num);
//                 // 遍历得到最远的右边界
//                 while (map.containsKey(right + 1)) {
//                     right = map.get(right + 1);
//                 }
//                 // 更新右边界
//                 map.put(num, right);
//                 // 更新答案
//                 ans = Math.max(ans, right - num + 1);
//             }
            
//         }
//         return ans;
//     }
// }
