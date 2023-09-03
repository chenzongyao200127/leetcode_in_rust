# 1921_Eliminate_Maximum_Number_of_Monsters
# https://leetcode.cn/problems/eliminate-maximum-number-of-monsters/

# 示例 1：
# 输入：dist = [1,3,4], speed = [1,1,1]
# 输出：3
# 解释：
# 第 0 分钟开始时，怪物的距离是 [1,3,4]，你消灭了第一个怪物。
# 第 1 分钟开始时，怪物的距离是 [X,2,3]，你没有消灭任何怪物。
# 第 2 分钟开始时，怪物的距离是 [X,1,2]，你消灭了第二个怪物。
# 第 3 分钟开始时，怪物的距离是 [X,X,1]，你消灭了第三个怪物。
# 所有 3 个怪物都可以被消灭。

# 示例 2：
# 输入：dist = [1,1,2,3], speed = [1,1,1,1]
# 输出：1
# 解释：
# 第 0 分钟开始时，怪物的距离是 [1,1,2,3]，你消灭了第一个怪物。
# 第 1 分钟开始时，怪物的距离是 [X,0,1,2]，你输掉了游戏。
# 你只能消灭 1 个怪物。

# 示例 3：
# 输入：dist = [3,2,4], speed = [5,3,2]
# 输出：1
# 解释：
# 第 0 分钟开始时，怪物的距离是 [3,2,4]，你消灭了第一个怪物。
# 第 1 分钟开始时，怪物的距离是 [X,0,2]，你输掉了游戏。 
# 你只能消灭 1 个怪物。

from typing import List

class Solution:
    def eliminateMaximum(self, dist: List[int], speed: List[int]) -> int:
        arive = []
        for i in range(len(dist)):
            if dist[i] % speed[i] == 0:
                arive.append(dist[i] // speed[i] - 1)
            else:
                arive.append(dist[i] // speed[i])
        arive.sort()
        ans = 0
        for i in range(len(arive)):
            if i <= arive[i]:
                ans += 1
            else:
                break
        
        return ans
        
        
        
        
        
s = Solution()
ans = s.eliminateMaximum(dist = [1,3,4], speed = [1,1,1])
print(ans)

s = Solution()
ans = s.eliminateMaximum(dist = [1,1,2,3], speed = [1,1,1,1])
print(ans)

s = Solution()
ans = s.eliminateMaximum(dist = [3,2,4], speed = [5,3,2])
print(ans)