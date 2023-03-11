// 80_Remove_Duplicates_from_Sorted_Array_II
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        for n in nums.clone() {
            if idx < 2 || n > nums[idx as usize - 2] {
                nums[idx as usize] = n;
                idx += 1;
            }
        }
        idx as i32
    }
}

// 双指针
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
}



// 通用解法
// 为了让解法更具有一般性，我们将原问题的「保留 2 位」修改为「保留 k 位」。

// 对于此类问题，我们应该进行如下考虑：

// 由于是保留 k 个相同数字，对于前 k 个数字，我们可以直接保留
// 对于后面的任意数字，能够保留的前提是：与当前写入的位置前面的第 k 个元素进行比较，不相同则保留
// class Solution {
//     public int removeDuplicates(int[] nums) {   
//         return process(nums, 2);
//     }
//     int process(int[] nums, int k) {
//         int u = 0; 
//         for (int x : nums) {
//             if (u < k || nums[u - k] != x) nums[u++] = x;
//         }
//         return u;
//     }
// }

// class Solution:
//     def removeDuplicates(self, nums: List[int]) -> int:
//         def solve(k):
//             u = 0
//             for x in nums:
//                 if u < k or nums[u - k] != x:
//                     nums[u] = x
//                     u += 1
//             return u
//         return solve(2)















// class Solution {
//     public int removeDuplicates(int[] nums) {
//         int i = 0;
//         for (int n : nums)
//             if (i < 2 || n > nums[i-2])
//                 nums[i++] = n;
//         return i;
//     }
// }