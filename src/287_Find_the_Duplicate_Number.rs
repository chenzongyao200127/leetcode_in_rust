// 287_Find_the_Duplicate_Number
// https://leetcode.cn/problems/find-the-duplicate-number/

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0 as usize;
        let mut fast = 0 as usize;
        loop {
            fast = nums[nums[fast] as usize] as usize;
            slow = nums[slow] as usize;
            if fast == slow {
                break;
            }
        }
        let mut fast = 0 as usize;
        loop {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
            if slow == fast {
                return slow as i32;
            }
        }
    }
}

/// 示例代码
/// 二分法
/// 这是一个 Rust 语言的函数，名为 `find_duplicate`。它接受一个整数类型的数组 `nums` 作为参数，并返回该数组中重复出现的数字。
/// 该函数使用二分查找算法来解决问题。
/// 首先，定义两个变量 `left` 和 `right` 分别表示可能存在重复数字的范围（从1到n-1），其中 n 是数组长度。
/// 然后，在 while 循环中进行二分查找：计算中间值 mid，遍历整个数组并统计小于等于 mid 的元素数量 cnt。
/// 如果 cnt 小于等于 mid，则说明重复元素在右侧区域，更新 left；否则在左侧区域，更新 right。
/// 最终循环结束时 left 即为所求结果。
/// 
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + ((right - left) >> 1);
            let mut cnt = 0;
            
            for i in 0..nums.len() {
                if nums[i] <= mid {
                    cnt += 1;
                }
            }

            if cnt <= mid {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}
