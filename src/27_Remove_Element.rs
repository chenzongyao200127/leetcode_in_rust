// 27. Remove Element
// https://leetcode.cn/problems/remove-element/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut idx = 0 as usize;
        while idx < nums.len() {
            if nums[idx] == val {
                nums.remove(idx);
                len -= 1;
            } else {
                idx += 1;
            }
        }
        len as i32
    }
}

// 有点类似快慢指针
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        for i in 0..nums.len() {
            if val != nums[i] {
                nums[index] = nums[i];
                index += 1;
            }
        }
        index as i32
    }
}

// int removeElement(int* nums, int numsSize, int val) {
//     int left = 0, right = numsSize;
//     while (left < right) {
//         if (nums[left] == val) {
//             nums[left] = nums[right - 1];
//             right--;
//         } else {
//             left++;
//         }
//     }
//     return left;
// }
// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/remove-element/solution/yi-chu-yuan-su-by-leetcode-solution-svxi/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。