// 26_Remove_Duplicates_from_Sorted_Array
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 1;
        }

        let mut change_cnt = 1 as usize;
        let mut cnt = 0;
        for i in 1..len {
            if nums[i] != nums[change_cnt-1] {
                nums[change_cnt] = nums[i];
                change_cnt += 1;
            } else {
                cnt += 1;
            }
        }
        len as i32 - cnt
    }
}

/// 示例代码
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slower = 1;
        for faster in 0..nums.len() - 1 {
            if nums[faster] != nums[faster + 1] {
                nums[slower] = nums[faster + 1];
                slower += 1;
            }
        }
        slower as i32
    }
}




impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 1 as usize;
        let len = nums.len();
        let mut cnt = 0;
        while idx < nums.len() {
            let pre = nums[idx-1];
            if nums[idx] == pre {
                nums.remove(idx);
                cnt += 1;
            } else {
                idx += 1;
            }
        }

        len as i32 - cnt
    }
}

