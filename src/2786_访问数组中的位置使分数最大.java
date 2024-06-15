// 2786_访问数组中的位置使分数最大
// https://leetcode.cn/problems/visit-array-positions-to-maximize-score/

class Solution {
    public long maxScore(int[] nums, int x) {
        long[] f = new long[2];
        for (int i = nums.length - 1; i >= 0; i--) {
            int v = nums[i];
            int r = v & 1; // 比 % 2 快一点
            f[r] = Math.max(f[r], f[r ^ 1] - x) + v;
        }
        return f[nums[0] & 1];
    }
}