# 565_Array_Nesting
# https://leetcode.cn/problems/array-nesting/

# time out
class Solution:
    def arrayNesting(self, nums: List[int]) -> int:
        res = 0
        for n in nums:
            tmp = []
            tmp.append(n)
            while nums[n] not in tmp:
                tmp.append(nums[n])
                n = nums[n]
            res = max(res, len(tmp))
        
        return res
    

class Solution:
    def arrayNesting(self, nums: List[int]) -> int:
        ans, n = 0, len(nums)
        vis = [False] * n
        for i in range(n):
            cnt = 0
            while not vis[i]:
                vis[i] = True
                i = nums[i]
                cnt += 1
            ans = max(ans, cnt)
            
        return ans
