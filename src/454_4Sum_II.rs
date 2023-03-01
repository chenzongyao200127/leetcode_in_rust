// 454. 4Sum II
// https://leetcode.cn/problems/4sum-ii/

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut ab_map: HashMap<i32, usize> = HashMap::new();
        let mut cd_map: HashMap<i32, usize> = HashMap::new();
        let mut ans = 0;
        let len = nums1.len();
        for i in 0..len {
            for j in 0..len {
                let tmp_add = nums1[i] + nums2[j];
                ab_map.entry(tmp_add).and_modify(|cnt| *cnt+=1).or_insert(1);
            }
        }
        for i in 0..len {
            for j in 0..len {
                let tmp_add = nums3[i] + nums4[j];
                cd_map.entry(tmp_add).and_modify(|cnt| *cnt+=1).or_insert(1);
            }
        }
        for (&k, &v) in cd_map.iter() {
            if let Some(val) = ab_map.get(&(-k)) {
                ans += val * v;
            }
        }
    
        ans as i32
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut hash = HashMap::new();
        nums1.iter().for_each(|x| {
            nums2.iter().for_each(|y| {
                *hash.entry(x+y).or_insert(0) += 1;
            })
        });
        nums3.iter().for_each(|x| {
            nums4.iter().for_each(|y| {
                if let Some(n) = hash.get(&(-x-y)) {
                    count += n;
                }
            })
        });
        count
    }
}