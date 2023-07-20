# 918_Maximum_Sum_Circular_Subarray
# https://leetcode.cn/problems/maximum-sum-circular-subarray/

# 给定一个长度为 n 的环形整数数组 nums ，返回 nums 的非空 子数组 的最大可能和 。

# 环形数组 意味着数组的末端将会与开头相连呈环状。
# 形式上， nums[i] 的下一个元素是 nums[(i + 1) % n] ， nums[i] 的前一个元素是 nums[(i - 1 + n) % n] 。

# 子数组 最多只能包含固定缓冲区 nums 中的每个元素一次。形式上，对于子数组 nums[i], nums[i + 1], ..., nums[j] ，
# 不存在 i <= k1, k2 <= j 其中 k1 % n == k2 % n 。
from typing import List


def maxSubarraySumCircular(nums: List[int]) -> int:
    def kadane(arr):
        curr_sum = max_sum = arr[0]
        for num in arr[1:]:
            curr_sum = max(num, curr_sum + num)
            max_sum = max(max_sum, curr_sum)
        return max_sum

    total = sum(nums)
    max_sum_wrap = total + kadane([-num for num in nums])
    
    # 注意如果所有数都是负数, maxSum_wrap = 0, 这种情况下应返回非环形部分的最大和
    return max(kadane(nums), max_sum_wrap) if max_sum_wrap != 0 else kadane(nums)

print(maxSubarraySumCircular([-1,-4,-5]))



from collections import deque
class Solution:
    def maxSubarraySumCircular(self, nums: List[int]) -> int:
        n = len(nums)
        q = deque()
        pre, res = nums[0], nums[0]
        q.append((0, pre))
        for i in range(1, 2 * n):
            while len(q) > 0 and q[0][0] < i - n:
                q.popleft()
            pre += nums[i % n]
            res = max(res, pre - q[0][1])
            while len(q) > 0 and q[-1][1] >= pre:
                q.pop()
            q.append((i, pre))
        return res
