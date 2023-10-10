// 540_有序数组中的单一元素
// https://leetcode.cn/problems/single-element-in-a-sorted-array/

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return nums[0];
        }

        if nums[0] != nums[1] {
            return nums[0];
        }

        if nums[len-1] != nums[len-2] {
            return nums[len-1];
        }

        #[inline]
        fn check(idx: usize, nums: &Vec<i32>) -> i32 {
            let pre = nums[idx - 1];
            let nex = nums[idx + 1]; 

            if pre == nums[idx] {
                if idx % 2 == 0 {
                    return 1;
                } else {
                    return -1;
                }
            }

            if nex == nums[idx] {
                if idx % 2 == 0 {
                    return -1;
                } else {
                    return 1;
                }
            }

            0
        }

        let mut l = 1;
        let mut r = len-2;
        let mut m = l + (r - l) / 2;

        while l < r {
            if check(m, &nums) > 0 {
                r = m;
            } else if check(m, &nums) < 0 {
                l = m;
            } else {
                break;
            }
            m = l + (r - l) / 2   
        }

        nums[m]
    }
}



// 优化
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;
            // Check if the index is even or odd and adjust mid accordingly
            let true_mid = if mid % 2 == 1 { mid - 1 } else { mid };
            
            if nums[true_mid] == nums[true_mid + 1] {
                // The single non-duplicate element is on the right side
                l = mid + 1;
            } else {
                // The single non-duplicate element is on the left side
                r = mid;
            }
        }

        nums[l]
    }
}
