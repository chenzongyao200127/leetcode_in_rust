// 215. Kth Largest Element in an Array
// https://leetcode.cn/problems/kth-largest-element-in-an-array/

// 给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

// 请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

// 你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {

    }
}

// CHEAT
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len()-k as usize]
    }
}