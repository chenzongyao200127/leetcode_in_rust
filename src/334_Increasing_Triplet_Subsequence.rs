// 334_Increasing_Triplet_Subsequence
// https://leetcode.cn/problems/increasing-triplet-subsequence/

// 334. 递增的三元子序列
// 给你一个整数数组 nums ，判断这个数组中是否存在长度为 3 的递增子序列。
// 如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k ，使得 nums[i] < nums[j] < nums[k] ，返回 true ；否则，返回 false 。

 
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut one = nums[0];
        let mut two = i32::MAX;
        for n in nums {
            // println!("(one, two, n): {:?}", (one, two, n));
            if n <= one.min(two) {
                one = n;
            } else if n > one && n <= two {
                two = n;
            } else if n > two && two > one {
                return true;
            } else {
                continue;
            }
        }

        false
    }
}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for n in nums {
            if n <= first {
                first = n;
            } else if n <= second {
                second = n;
            } else {
                return true;
            }
        }
        false
    }
}