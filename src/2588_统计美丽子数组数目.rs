// 2588_统计美丽子数组数目
// https://leetcode.cn/problems/count-the-number-of-beautiful-subarrays/

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut s = 0;
        let mut cnt = HashMap::with_capacity(nums.len() + 1);
        cnt.insert(0, 1);
        for n in nums {
            s ^= n;
            let e = cnt.entry(s).or_insert(0);
            ans += *e as i64;
            *e += 1;
        }
        ans
    }
}
