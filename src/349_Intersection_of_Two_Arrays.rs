// 349. Intersection of Two Arrays
// https://leetcode.cn/problems/intersection-of-two-arrays/

// 方法一：两个Hash集合
// 方法二：排序 + 双指针
use std::collections::HashSet;
impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();
        let (mut i, mut j) = (0, 0);
        let mut result = HashSet::new();
        while i < nums1.len() && j <  nums2.len() {
            let (n1, n2) = (nums1[i], nums2[j]);
            if n1 == n2 {
                result.insert(n1);
                i += 1;
                j += 1;
            } else if n1 > n2 {
                j += 1;
            } else {
                i += 1;
            }
        }
        result.into_iter().collect()
    }
}


use std::collections::{HashSet};

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut H: HashSet<i32> = HashSet::new();
        let mut ans = vec![];
        for i in nums1.into_iter() {
            if nums2.contains(&i) {
                ans.push(i);
            }
        }
        for i in ans {
            if let Some(_) = H.get(&i) {
                continue;
            }
            H.insert(i);
        }

        H.into_iter().collect()
    }
}



use std::collections::{HashSet};
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut H: HashSet<i32> = HashSet::new();
        let mut ans = vec![];
        for i in nums1.into_iter() {
            if nums2.contains(&i) {
                ans.push(i);
            }
        }
        let mut result = vec![];
        for i in ans {
            if let Some(_) = H.get(&i) {
                continue;
            }
            H.insert(i);
            result.push(i);
        }

        result
    }
}

        // pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        //     let mut H:HashSet<i32>=HashSet::with_capacity(nums.len());
        //     for i in nums {
        //         if let Some(_) = H.get(&i) {
        //             return true
        //         }
        //         H.insert(i);
        //     }
        //     false
        // }


    // class Solution {
    //     public int[] intersection(int[] nums1, int[] nums2) {
    //         Set<Integer> set1 = new HashSet<Integer>();
    //         Set<Integer> set2 = new HashSet<Integer>();
    //         for (int num : nums1) {
    //             set1.add(num);
    //         }
    //         for (int num : nums2) {
    //             set2.add(num);
    //         }
    //         return getIntersection(set1, set2);
    //     }
    
    //     public int[] getIntersection(Set<Integer> set1, Set<Integer> set2) {
    //         if (set1.size() > set2.size()) {
    //             return getIntersection(set2, set1);
    //         }
    //         Set<Integer> intersectionSet = new HashSet<Integer>();
    //         for (int num : set1) {
    //             if (set2.contains(num)) {
    //                 intersectionSet.add(num);
    //             }
    //         }
    //         int[] intersection = new int[intersectionSet.size()];
    //         int index = 0;
    //         for (int num : intersectionSet) {
    //             intersection[index++] = num;
    //         }
    //         return intersection;
    //     }
    // }
    
    // 作者：LeetCode-Solution
    // 链接：https://leetcode.cn/problems/intersection-of-two-arrays/solution/liang-ge-shu-zu-de-jiao-ji-by-leetcode-solution/
    // 来源：力扣（LeetCode）
    // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。