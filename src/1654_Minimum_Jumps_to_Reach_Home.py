# 1654_Minimum_Jumps_to_Reach_Home
# https://leetcode.cn/problems/minimum-jumps-to-reach-home/

# 有一只跳蚤的家在数轴上的位置 x 处。请你帮助它从位置 0 出发，到达它的家。

# 跳蚤跳跃的规则如下：

# 它可以 往前 跳恰好 a 个位置（即往右跳）。
# 它可以 往后 跳恰好 b 个位置（即往左跳）。
# 它不能 连续 往后跳 2 次。
# 它不能跳到任何 forbidden 数组中的位置。
# 跳蚤可以往前跳 超过 它的家的位置，但是它 不能跳到负整数 的位置。

# 给你一个整数数组 forbidden ，其中 forbidden[i] 是跳蚤不能跳到的位置，同时给你整数 a， b 和 x ，
# 请你返回跳蚤到家的最少跳跃次数。如果没有恰好到达 x 的可行方案，请你返回 -1 。

# 示例 1：
# 输入：forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9
# 输出：3
# 解释：往前跳 3 次（0 -> 3 -> 6 -> 9），跳蚤就到家了。

# 示例 2：
# 输入：forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11
# 输出：-1

# 示例 3：
# 输入：forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7
# 输出：2
# 解释：往前跳一次（0 -> 16），然后往回跳一次（16 -> 7），跳蚤就到家了。

# 1 <= forbidden.length <= 1000
# 1 <= a, b, forbidden[i] <= 2000
# 0 <= x <= 2000
# forbidden 中所有位置互不相同。
# 位置 x 不在 forbidden 中。

from typing import List
from collections import deque

class Solution:
    def minimumJumps(self, forbidden: List[int], a: int, b: int, x: int) -> int:
        max_val = max(x, max(forbidden)) + a + b  
        forbidden = set(forbidden)
        
        q = deque([(0, 1, 0)])
        visited = set()
        visited.add((0, 1))
        
        while q:
            cur_pos, pre_dir, times = q.popleft()
            
            if cur_pos == x:
                return times
            
            if 0 <= cur_pos + a <= max_val and (cur_pos + a, 1) not in visited and cur_pos + a not in forbidden:
                visited.add((cur_pos + a, 1))
                q.append((cur_pos + a, 1, times + 1))

            if pre_dir != -1 and 0 <= cur_pos - b and (cur_pos - b, -1) not in visited and cur_pos - b not in forbidden:
                visited.add((cur_pos - b, -1))
                q.append((cur_pos - b, -1, times + 1))

        return -1

        
s = Solution()
ans = s.minimumJumps(forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9)
print(ans)

s = Solution()
ans = s.minimumJumps(forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11)
print(ans)

s = Solution()
ans = s.minimumJumps(forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7)
print(ans)

s = Solution()
ans = s.minimumJumps(forbidden = [162,118,178,152,167,100,40,74,199,186,26,73,200,127,30,124,193,84,184,36,103,149,153,9,54,154,133,95,45,198,79,157,64,122,59,71,48,177,82,35,14,176,16,108,111,6,168,31,134,164,136,72,98], a = 29, b = 98, x = 80)
print(ans)



from collections import deque

def minimumJumps(forbidden, a, b, x):
    max_val = max(x, max(forbidden)) + a + b  
    forbidden_set = set(forbidden)
    visited = set([(0, 0)])  

    queue = deque([(0, 0, 0)]) 
    while queue:
        pos, direction, steps = queue.popleft()

        if pos == x:
            return steps

        if pos + a <= max_val and (pos + a, 1) not in visited and pos + a not in forbidden_set:
            visited.add((pos + a, 1))
            queue.append((pos + a, 1, steps + 1))

        if direction != -1 and 0 <= pos - b and (pos - b, -1) not in visited and pos - b not in forbidden_set:
            visited.add((pos - b, -1))
            queue.append((pos - b, -1, steps + 1))

    return -1

# 测试
forbidden = [2, 5, 8]
a = 3
b = 2
x = 9
print(minimumJumps(forbidden, a, b, x))  # 输出: 3 (跳过2,跳到6,再跳到3,再跳到9)
