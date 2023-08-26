# 228_Summary_Ranges
# https://leetcode.cn/problems/summary-ranges/description/

from typing import List
class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        if not nums:
            return []
            
        l = 0
        r = 0
        ans = []
        while r < len(nums)-1:
            if nums[r + 1] == nums[r] + 1:
                r += 1
            else:
                if l == r:
                    ans.append(str(nums[l]))
                else:
                    ans.append(str(nums[l]) + "->" + str(nums[r]))
                l = r + 1
                r = l
        if l == r:
            ans.append(str(nums[l]))
        else:
            ans.append(str(nums[l]) + "->" + str(nums[r]))
    
        return ans
    

# GPT-4 优化
from typing import List

class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        if not nums:
            return []
        
        # Start and end pointers for ranges.
        start, end = 0, 0
        result = []
        
        # A helper function to add the current range to result.
        def addRange(start: int, end: int):
            if start == end:
                result.append(str(nums[start]))
            else:
                result.append(f"{nums[start]}->{nums[end]}")
        
        for i in range(1, len(nums)):
            # Check if the current number is consecutive to the previous one.
            if nums[i] == nums[i - 1] + 1:
                end = i
            else:
                addRange(start, end)
                start = end = i
        
        # Add the last range.
        addRange(start, end)
        
        return result

        
# 零神的题解

# 官解的代码是我之前讲周赛时提到的「分组循环」。
# 这个写法的好处是，无需特判 nums是否为空，也无需在循环结束后，再补上处理最后一段区间的逻辑。
# 以我的经验，这种写法是所有写法中最不容易出 bug 的，推荐大家记住。
# 适用场景：⭐按照题目要求，数组会被分割成若干段，且每一段的判断/处理逻辑是一样的。
# 注：虽然代码写的是一个二重循环，但 i += 1 这句话至多执行 n 次，所以总的时间复杂度仍然是 O(n) 的。

class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        ans = []
        i, n = 0, len(nums)
        while i < n:
            start = i
            while i < n - 1 and nums[i] + 1 == nums[i + 1]:
                i += 1
            s = str(nums[start])
            if start < i:
                s += "->" + str(nums[i])
            ans.append(s)
            i += 1
            
        return ans


# 一般来说，分组循环的模板如下（根据题目调整）：
# i, n = 0, len(nums)
# while i < n:
#     start = i
#     while i < n and ...:
#         i += 1
#     # 从 start 到 i-1 是一段
#     # 下一段从 i 开始，无需 i+=1