# 698_划分为k个相等的子集
# https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/description/

# 给定一个整数数组  nums 和一个正整数 k，找出是否有可能把这个数组分成 k 个非空子集，其总和都相等。

# 示例 1：
# 输入： nums = [4, 3, 2, 3, 5, 2, 1], k = 4
# 输出： True
# 说明： 有可能将其分成 4 个子集（5），（1,4），（2,3），（2,3）等于总和。

# 示例 2:
# 输入: nums = [1,2,3,4], k = 3
# 输出: false
 
# 提示：
# 1 <= k <= len(nums) <= 16
# 0 < nums[i] < 10000
# 每个元素的频率在 [1,4] 范围内

from typing import List

# 超时
class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        n = len(nums)
        nums.sort(reverse=True)
        
        if sum(nums) % k != 0:
            return False
        
        target = sum(nums) // k
        
        for num in nums:
            if num > target:
                return False
        
        # Initially, the mask is 0 because no elements have been used
        mask = 0

        def dfs(mask, cur_val, cur_set_num):
            if cur_set_num == k and mask == (1 << n) - 1:  # All the bits should be set for all elements
                return True

            if cur_val == target:
                return dfs(mask, 0, cur_set_num + 1)  # Proceed with next subset

            for i in range(n-1, -1, -1):
                if not mask & (1 << i) and cur_val + nums[i] <= target:
                    if dfs(mask | (1 << i), cur_val + nums[i], cur_set_num):  # Set the bit for the ith element and add its value to cur_val
                        return True

            return False
        
        return dfs(mask, 0, 0)



#记忆化搜索优化
from typing import List

class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        n = len(nums)
        nums.sort(reverse=True)
        
        if sum(nums) % k != 0:
            return False
        
        target = sum(nums) // k
        
        for num in nums:
            if num > target:
                return False
        
        mask = 0  # Initially, the mask is 0 because no elements have been used
        memo = {}  # Store results of previous computations
        
        def dfs(mask, cur_val, cur_set_num):
            if (mask, cur_val, cur_set_num) in memo:
                return memo[(mask, cur_val, cur_set_num)]

            if cur_set_num == k and mask == (1 << n) - 1:
                return True

            if cur_val == target:
                return dfs(mask, 0, cur_set_num + 1)

            for i in range(n-1, -1, -1):
                if not mask & (1 << i) and cur_val + nums[i] <= target:
                    if dfs(mask | (1 << i), cur_val + nums[i], cur_set_num):
                        memo[(mask, cur_val, cur_set_num)] = True
                        return True

            memo[(mask, cur_val, cur_set_num)] = False
            return False
        
        return dfs(mask, 0, 0)

        
s = Solution()
ans = s.canPartitionKSubsets(nums = [4, 3, 2, 3, 5, 2, 1], k = 4)
print(ans)

s = Solution()
ans = s.canPartitionKSubsets(nums = [1,2,3,4], k = 3)
print(ans)

s = Solution()
ans = s.canPartitionKSubsets(nums = [724,3908,1444,522,325,322,1037,5508,1112,724,424,2017,1227,6655,5576,543], k = 4)
print(ans)



# 状态压缩DP
# Using Dynamic Programming (DP) for this problem involves recognizing 
# that it's essentially a variation of the subset sum problem but with `k` subsets. 
# We'll use the mask to represent the state of the subset and use DP to store and compute values for each state.

# Here's how we can use DP:

# 1. Create a `dp` array of size `(1 << n)` (where `n` is the length of `nums`) to represent all possible states.
# 2. Initialize `dp[0]` to 0 because the sum of the empty set is 0.
# 3. Iterate through each state and update the `dp` values based on previous states.

# Here's the implementation:


from typing import List

class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        n = len(nums)
        if sum(nums) % k != 0:
            return False

        target = sum(nums) // k

        # Create a dp array of size 2^n with all values set to -1 (unvisited)
        dp = [-1] * (1 << n)
        dp[0] = 0  # Starting with no elements, the sum is 0

        for mask in range(1 << n):
            if dp[mask] == -1: 
                continue  # If this state was not achieved by any previous states, skip

            for j in range(n):
                nex_mask = mask | (1 << j)  # Try to add nums[j] to current subset
                if not mask & (1 << j) and dp[mask] + nums[j] <= target:
                    # Update the sum of the subset for nex_mask state
                    dp[nex_mask] = (dp[mask] + nums[j]) % target

        # If the last state's value is 0, it means the array can be partitioned into k subsets
        return dp[-1] == 0

# The core idea of this DP solution is to explore all possible combinations (subsets) 
# of the `nums` array and compute their cumulative sums. 
# The resulting sums are then stored in the `dp` table, a
# nd the final state indicates whether the array can be partitioned into `k` subsets.        