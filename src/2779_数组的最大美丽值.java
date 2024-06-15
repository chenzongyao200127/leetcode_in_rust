// 2779_数组的最大美丽值
// https://leetcode.cn/problems/maximum-beauty-of-an-array-after-applying-operation/description/?envType=daily-question&envId=2024-06-15

import java.util.Arrays;

class Solution {
    public int maximumBeauty(int[] nums, int k) {
        Arrays.sort(nums);
        int ans = 0;
        int left = 0;
        for (int right = 0; right < nums.length; right++) {
            while (nums[right] - nums[left] > k * 2) {
                left++;
            }
            ans = Math.max(ans, right - left + 1);
        }
        return ans;
    }
}