// 2460_Apply_Operations_to_an_Array
// https://leetcode.cn/problems/apply-operations-to-an-array/

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        let mut j = 0;

        for i in 0..len {
            if i + 1 < len && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }

        nums
    }
}

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        for i in 0..len - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        for i in 0..len {
            for j in 0..len - 1 {
                if nums[j] == 0 && nums[j + 1] != 0 {
                    nums.swap(j, j + 1);
                }
            }
        }
        nums
    }
}

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut nums = nums.clone();
        while (i < nums.len() - 1) {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            i += 1;
        }
        i = 0;
        let mut t = 0;
        while (i < nums.len()) {
            while (i < nums.len() && nums[i] != 0) {
                i += 1;
            }
            t = i + 1;
            while (t < nums.len() && nums[t] == 0) {
                t += 1;
            }
            if i >= nums.len() || t >= nums.len() {
                break;
            }
            if i < t {
                let x = nums[i];
                nums[i] = nums[t];
                nums[t] = x;
                i += 1;
                t += 1;
                if i >= nums.len() || t >= nums.len() {
                    break;
                }
            }
        }
        return nums;
    }
}
