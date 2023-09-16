// 34_Find_First_and_Last_Position_of_Element_in_Sorted_Array
// https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/description/

// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
// If target is not found in the array, return [-1, -1].
// You must write an algorithm with O(log n) runtime complexity.

 
// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]

// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]

// Example 3:
// Input: nums = [], target = 0
// Output: [-1,-1]

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];

        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut mid = l + (r - l) / 2;

        if nums.is_empty() {
            return ans;
        }

        if nums[0] > target {
            return ans;
        }

        if nums[r] < target {
            return ans;
        }
        
        while l <= r {
            if nums[mid] == target {
                let mut tmp = mid as i32;
                while tmp >= 0 && nums[tmp as usize] == target {
                    tmp -= 1;
                }
                ans[0] = tmp as i32 + 1;

                tmp = mid as i32;
                while tmp < nums.len() as i32 - 1 && nums[tmp as usize] == target {
                    tmp += 1;
                }
                if tmp as usize == mid {
                    ans[1] = tmp as i32;
                } else {
                    if tmp == nums.len() as i32 - 1 && nums[tmp as usize] == target {
                        ans[1] = tmp as i32;
                    } else {
                        ans[1] = tmp as i32 - 1;
                    }
                }

                break;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }

            if r < l {
                break;
            }
            mid = l + (r - l) / 2;
        }

        ans
    }
}

// 代码优化
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = Self::find_first(&nums, target);
        let end = Self::find_last(&nums, target);
        vec![start, end]
    }

    // Helper function to find the first occurrence of target
    fn find_first(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] < target {
                l = mid + 1;
            } else if nums[mid as usize] > target {
                r = mid - 1;
            } else {
                if mid == 0 || nums[mid as usize - 1] != target {
                    return mid;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }

    // Helper function to find the last occurrence of target
    fn find_last(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] < target {
                l = mid + 1;
            } else if nums[mid as usize] > target {
                r = mid - 1;
            } else {
                if mid == nums.len() as i32 - 1 || nums[mid as usize + 1] != target {
                    return mid;
                } else {
                    l = mid + 1;
                }
            }
        }
        -1
    }
}


// 偷懒
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let v:Vec<(usize, &i32)> = nums.iter().enumerate().filter(|(_i, &x)| {
            x == target
        }).collect();

        if v.len() == 0 {
            return vec![-1, -1];
        }

        vec![v[0].0 as i32, v[v.len() -1].0 as i32]
    }
}
