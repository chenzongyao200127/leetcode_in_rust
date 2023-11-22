// 153_Find_Minimum_in_Rotated_Sorted_Array
// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/description/

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        if nums[0] < nums[nums.len()-1] {
            return nums[0];
        }

        if nums[nums.len()-1] < nums[0] && nums[0] <= nums[nums.len()-2] {
            return nums[nums.len()-1];
        }

        #[inline]
        fn check(nums: &[i32], idx: usize) -> bool {
            if nums[idx] >= nums[0] {
                true
            } else {
                false
            }
        }

        let mut l = 0;
        let mut r = nums.len() - 1; 

        while l < r {
            let mid = l + (r - l) / 2;
            // println!("{:?}", (l, r, mid));
            if check(&nums, mid) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        return nums[l]
    }
}

// 优化
pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
            // Minimum element must be to the right of mid
            left = mid + 1;
        } else {
            // Minimum element is not to the right of mid
            right = mid;
        }
    }

    nums[left]
}
