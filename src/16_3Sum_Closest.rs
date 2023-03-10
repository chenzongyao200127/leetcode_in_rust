// 16_3Sum_Closest
// https://leetcode.cn/problems/3sum-closest/

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        if len == 3 {
            return nums.iter().sum::<i32>();
        }

        nums.sort_unstable();
        
        let m = nums[0..3].iter().sum();
        if m >= target {
            return m;
        }
        let x = nums[nums.len() - 3..nums.len()].iter().sum();
        if x <= target {
            return x;
        }

        let mut diff = i32::MAX;
        for i in 0..len-2 {
            let t = target - nums[i];
            let mut left = i+1;
            let mut right = len-1;
            while left < right {
                if nums[left] + nums[right] < t {
                    if (nums[left] + nums[right] - t).abs() < diff.abs() {
                        diff = nums[left] + nums[right] - t;
                    }
                    left += 1;
                } else if nums[left] + nums[right] > t {
                    if (nums[left] + nums[right] - t).abs() < diff.abs() {
                        diff = nums[left] + nums[right] - t;
                    }
                    right -= 1
                } else {
                    return target;
                }
            }
        }
        return target + diff

    }
}


impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let m = nums[0..3].iter().sum();
        if m >= target {
            return m;
        }
        let x = nums[nums.len() - 3..nums.len()].iter().sum();
        if x <= target {
            return x;
        }

        let mut c = if target - m > x - target { m } else { x };
        for i in 0..nums.len() - 1 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let s = nums[i] + nums[l] + nums[r];
                if s < target {
                    if target - s < (target - c).abs() {
                        c = s;
                    }
                    l += 1;
                } else if s > target {
                    if s - target < (c - target).abs() {
                        c = s;
                    }
                    r -= 1;
                } else {
                    return target;
                }
            }
        }

        c
    }
}