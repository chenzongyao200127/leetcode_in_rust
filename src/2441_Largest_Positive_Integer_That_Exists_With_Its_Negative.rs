// 2441_Largest_Positive_Integer_That_Exists_With_Its_Negative
// https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative/


impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut filter_nums: Vec<_> = nums.iter().filter(|&x| nums.contains(&(-x))).collect();
        filter_nums.sort_unstable();
        if filter_nums.len() == 0 { return -1 } else { (*filter_nums[0]).abs() }
    }
}


impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut s = std::collections::HashSet::new();
        let mut ans = -1;

        for num in nums {
            if s.contains(&(-num)) {
                ans = ans.max(num.abs());
            }

            s.insert(num);
        }

        ans
    }
}
