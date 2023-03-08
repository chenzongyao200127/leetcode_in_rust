// 496_Next_Greater_Element_I
// https://leetcode.cn/problems/next-greater-element-i/

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let len1 = nums1.len();
        let len2 = nums2.len();
        for i in 0..len1 {
            for j in 0..len2 {
                if nums2[j] == nums1[i] {
                    if j == len2-1 {
                        ans.push(-1);
                    } else {
                        for k in j+1..len2 {
                            if nums2[k] > nums1[i] {
                                ans.push(nums2[k]);
                                break;
                            } else {
                                if k == len2-1 {
                                    ans.push(-1);
                                }
                            }                        
                        }
                    }
                    break;
                }
            }
        }

        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let len2 = nums2.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut stack = vec![];
        // 单调栈
        for i in (0..len2).rev() {
            let num = nums2[i];
            while !stack.is_empty() && num >= stack[stack.len()-1] {
                stack.pop();
            }
            if !stack.is_empty() {
                map.insert(num, stack[stack.len() - 1]);    
            } else {
                map.insert(num, -1);
            }
            stack.push(num);
        }

        for num in nums1 {
            ans.push(*map.get(&num).unwrap());
        }

        ans
    }
}


impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.into_iter().fold(vec![], |mut acc, x| {
            let mut pos = 0;
            while pos < nums2.len() && x != nums2[pos] {
                pos += 1;
            }
            if pos == nums2.len() {
                acc.push(-1);
            } else {
                while pos < nums2.len() && nums2[pos] <= x {
                    pos += 1;
                }
                if pos == nums2.len() {
                    acc.push(-1);
                } else {
                    acc.push(nums2[pos]);
                }
            }
            acc
        })
    }
}