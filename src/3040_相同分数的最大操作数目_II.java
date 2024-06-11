// 3040_相同分数的最大操作数目_II
// https://leetcode.cn/problems/maximum-number-of-operations-with-the-same-score-ii/description/?envType=daily-question&envId=2024-06-08

// 3038. 相同分数的最大操作数目 I
class Solution {
    public int maxOperations(int[] nums) {
        int s = nums[0] + nums[1];
        int ans = 1;
        for (int i = 3; i < nums.length && nums[i - 1] + nums[i] == s; i += 2) {
            ans++;
        }
        return ans;
    }
}