// 525. Contiguous Array
// https://leetcode.cn/problems/contiguous-array/


// Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.
use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut compare = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                compare += 1;
            } else {
                compare -= 1;
            }
            if let Some(val) = map.get(&compare) {
                ans = ans.max(i as i32 - *val);
            } else {
                map.insert(compare, i as i32);
            }
        }
        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let one_nums = nums.iter().sum::<i32>();
        let zero_nums: i32 = nums.len() as i32 - one_nums;
        let mut compare = -zero_nums + one_nums;
        map.insert(compare, nums.len() as i32-1);
        for i in (0..nums.len()).rev() {
            if nums[i] == 1 {
                compare -= 1;
            } else {
                compare += 1;
            }
            if let Some(_) = map.get(&compare) {
                continue;
            } else {
                map.insert(compare, i as i32 - 1);
            }
        }
        // println!("{:?}",map);

        let mut compare = 0;
        let mut idx = -1;
        let mut ans = 0;
        for i in 0..nums.len() {
            if let Some(val) = map.get(&compare) {
                ans = ans.max(val - idx);
            }
            if nums[i] == 1 {
                compare += 1;
            } else {
                compare -= 1;
            }
            idx += 1;
        }

        ans
    }
}

use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let mut l: i32 = 0;
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(0, -1);
        nums.iter().enumerate().for_each(|(i, x)| {
            if *x == 1 {
                counter += 1;
            } else {
                counter -= 1;
            }

            let firstIdx = *m.entry(counter).or_insert(i as i32);
            l = max(l, i as i32 - firstIdx);
        });
        l
    }
}