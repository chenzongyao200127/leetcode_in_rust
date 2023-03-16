use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let ans = find_radius(vec![1,3,5,9,12], vec![4,8]);
    assert_eq!(ans, 4);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p = nums1.len() - 1;
    let mut p1 = nums1.len() - m as usize - 1;
    let mut p2 = nums2.len() - 1;
    while nums1[p1] != 0 && nums2[p2] != 0 {
        if nums1[p1] > nums2[p2] {
            nums1[p] = nums1[p1];
            p -= 1;
            p1 -= 1;
        } else {
            nums1[p] = nums2[p2];
            p -= 1;
            p2 -= 1;
        }
    }

    for i in 0..p2 {
        nums1[i] = nums2[i];
    }
}