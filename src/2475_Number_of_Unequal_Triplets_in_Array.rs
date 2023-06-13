// 2475_Number_of_Unequal_Triplets_in_Array
// https://leetcode.cn/problems/number-of-unequal-triplets-in-array/

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        if nums.len() < 2 {
            return 0
        }
        for i in 0..nums.len()-2 {
            for j in i..nums.len()-1 {
                for k in j..nums.len() {
                    if nums[i] != nums[j] && nums[i] != nums[k] && nums[j] != nums[k] {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}



impl Solution {
    pub fn unequal_triplets(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut i = 0;
        let mut ans = 0;

        for j in 0..n-1 {
            if nums[j] != nums[j+1] {
                ans += i * (j - i + 1) * (n - j - 1);
                i = j + 1;
            }
        }

        ans as i32
    }
}