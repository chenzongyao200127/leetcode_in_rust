# 2342_数位和相等数对的最大和
# https://leetcode.cn/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/

from typing import List

class Solution:
    def maximumSum(self, nums: List[int]) -> int:
        nums = sorted(nums)
        # print(nums)
        
        cnt = [[] for _ in range(32 * 9)]
        def digits(n):
            ans = 0
            while n >= 10:
                ans += n % 10
                n = n // 10
            ans += n    
            # print(n)
            return ans
        
        ans = -1
        for i in range(len(nums)-1, -1, -1):
            cur_cnt = cnt[digits(nums[i])]
            cur_cnt.append(nums[i])
            if len(cur_cnt) == 2:
                ans = max(ans, sum(cur_cnt))
        
        return ans
    
s = Solution()
ans = s.maximumSum([18,43,36,13,7])
print(ans)


class Solution:
    def maximumSum(self, nums: List[int]) -> int:
        def digit_sum(n):
            return sum(map(int, str(n)))

        max_nums = {}

        ans = -1
        for num in nums:
            d_sum = digit_sum(num)
            if d_sum not in max_nums:
                max_nums[d_sum] = [num]
            else:
                if len(max_nums[d_sum]) < 2:
                    max_nums[d_sum].append(num)
                    if len(max_nums[d_sum]) == 2:
                        ans = max(ans, sum(max_nums[d_sum]))
                else:
                    max_nums[d_sum].sort(reverse=True)
                    if num > max_nums[d_sum][1]:
                        max_nums[d_sum][1] = num
                        ans = max(ans, sum(max_nums[d_sum]))

        return ans