# https://leetcode.cn/problems/split-with-minimum-sum/

class Solution:
    def splitNum(self, num: int) -> int:
        nums = sorted(list(filter(lambda x: x != '0', list(str(num)))))
        n1, n2 = 0, 0
        for i, n in enumerate(nums):
            n = int(n)
            if i % 2 == 0:
                n1 = n1 * 10 + n
            else:
                n2 = n2 * 10 + n
                
        return n1 + n2
        
        
s = Solution()
print(s.splitNum(430250))