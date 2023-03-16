// 88_Merge_Sorted_Array
// https://leetcode.cn/problems/merge-sorted-array/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m as usize..nums1.len() {
            nums1[i] = nums2[i-m as usize];
        }
        nums1.sort_unstable()
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p = (n + m) - 1;
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }

        if p2 >= 0 {
            for i in 0..=p2 as usize {
                nums1[i] = nums2[i];
            }
        }
    }
}