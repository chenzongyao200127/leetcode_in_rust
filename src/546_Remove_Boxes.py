#  546_Remove_Boxes
#  https://leetcode.cn/problems/remove-boxes/

# 给出一些不同颜色的盒子 boxes ，盒子的颜色由不同的正数表示。

# 你将经过若干轮操作去去掉盒子，直到所有的盒子都去掉为止。
# 每一轮你可以移除具有相同颜色的连续 k 个盒子（k >= 1），这样一轮之后你将得到 k * k 个积分。

# 返回 你能获得的最大积分和 。

# 示例 1：
# 输入：boxes = [1,3,2,2,2,3,4,3,1]
# 输出：23
# 解释：
# [1, 3, 2, 2, 2, 3, 4, 3, 1] 
# ----> [1, 3, 3, 4, 3, 1] (3*3=9 分) 
# ----> [1, 3, 3, 3, 1] (1*1=1 分) 
# ----> [1, 1] (3*3=9 分) 
# ----> [] (2*2=4 分)

# 示例 2：
# 输入：boxes = [1,1,1]
# 输出：9

# 示例 3：
# 输入：boxes = [1]
# 输出：1

from typing import List

class Solution:
    def removeBoxes(self, boxes: List[int]) -> int:
        dp = [[[0] * 100 for _ in range(100)] for _ in range(100)]
        
        def calculatePoints(boxes, l, r, k):
            if l > r:
                return 0
            
            if dp[l][r][k] == 0:
                dp[l][r][k] = calculatePoints(boxes, l, r - 1, 0) + (k + 1) * (k + 1)
            
                for i in range(l, r):
                    if boxes[i] == boxes[r]:
                        dp[l][r][k] = max(dp[l][r][k], calculatePoints(boxes, l, i, k+1) + calculatePoints(boxes, i+1, r-1, 0))
                        
            return dp[l][r][k]
            
        
        calculatePoints(boxes, 0, len(boxes) - 1, 0)
        return dp[0][len(boxes)-1][0]    
        
        
        
        
s = Solution()
ans = s.removeBoxes([1,3,2,2,2,3,4,3,1])
print(ans)        


s = Solution()
ans = s.removeBoxes([1,1,1])
print(ans)


s = Solution()
ans = s.removeBoxes([1])
print(ans)