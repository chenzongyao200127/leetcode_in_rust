// 55_Jump_Game
// https://leetcode.cn/problems/jump-game/description/

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut end = 0;
        for i in 0..n {
            if i <= end {
                end = end.max(i + nums[i] as usize)
            }
            if end >= n - 1 {
                return true;
            }
        }
        false
    }
}