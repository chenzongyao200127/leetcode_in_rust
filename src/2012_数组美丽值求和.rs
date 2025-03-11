// 2012_数组美丽值求和
// https://leetcode.cn/problems/sum-of-beauty-in-the-array/description/?envType=daily-question&envId=2025-03-11
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_min = vec![0; n];

        left_max[0] = nums[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(nums[i]);
        }

        right_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            right_min[i] = right_min[i + 1].min(nums[i]);
        }

        let mut ans = 0;
        for i in 1..n - 1 {
            if left_max[i - 1] < nums[i] && nums[i] < right_min[i + 1] {
                ans += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                ans += 1;
            }
        }

        ans
    }
}
