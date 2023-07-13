# 45_Jump_Game_II
# https://leetcode.cn/problems/jump-game-ii/

# 给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。

# 每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:

# 0 <= j <= nums[i] 
# i + j < n
# 返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。

# 输入: nums = [2,3,1,1,4]
# 输出: 2
# 解释: 跳到最后一个位置的最小跳跃数是 2。
#      从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 
from typing import List


class Solution:
    def jump(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return 0
        
        n = len(nums)
        start = 0
        end = nums[0]
        dp = [0] * n
        while True:
            tmp = 0
            for i in range(start+1, min(end+1, n+1)):
                tmp = max(tmp, nums[i] + i)
                dp[i] = dp[start] + 1
                if i == n-1:
                    return dp[i]    
            start = end
            end = tmp
            
# 优化写法            
class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return 0
        
        start, end, steps = 0, nums[0], 0
        while end < n-1:
            steps += 1
            max_reach = max(i+nums[i] for i in range(start, end+1))
            start, end = end, max_reach
        
        return steps+1

            
            
            
    