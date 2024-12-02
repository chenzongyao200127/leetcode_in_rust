from collections import defaultdict
from typing import List


class Solution:
    def hasIncreasingSubarrays(self, nums: List[int], k: int) -> bool:
        n = len(nums)
        dp = [1] * n
        for i in range(1, n):
            if nums[i] > nums[i - 1]:
                dp[i] = dp[i - 1] + 1
            else:
                dp[i] = 1
            print(dp)
            if i >= k and dp[i] >= k and dp[i - k] >= k:
                return True
        return False


class Solution:
    def maxIncreasingSubarrays(self, nums: List[int]) -> int:

        n = len(nums)
        dp = [1] * n
        for i in range(1, n):
            if nums[i] > nums[i - 1]:
                dp[i] = dp[i - 1] + 1
            else:
                dp[i] = 1
        # find the maximum k, use binary search
        res = 0
        left, right = 0, n//2 + 1
        while left < right:
            mid = (left + right + 1) // 2
            flag = False
            for i in range(n - mid):
                if dp[i] >= mid and dp[i + mid] >= mid:
                    flag = True
                    break
            if flag:
                left = mid
                res = mid
            else:
                right = mid - 1
        return res


class Solution:
    def sumOfGoodSubsequences(self, nums: List[int]) -> int:
        MOD = 10**9 + 7
        dp_map = defaultdict(int)
        cnt_map = defaultdict(int)

        total_sum = 0

        for num in nums:
            current_dp = num
            current_cnt = 1

            if num - 1 in dp_map:
                current_cnt += cnt_map[num - 1]
                current_dp += (dp_map[num - 1] + num * cnt_map[num - 1]) % MOD

            if num + 1 in dp_map:
                current_cnt += cnt_map[num + 1]
                current_dp += (dp_map[num + 1] + num * cnt_map[num + 1]) % MOD

            dp_map[num] = (dp_map[num] + current_dp) % MOD
            cnt_map[num] = (cnt_map[num] + current_cnt) % MOD

            total_sum = (total_sum + current_dp) % MOD

        return total_sum


