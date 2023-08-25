# 1267_Count_Servers_that_Communicate
# https://leetcode.cn/problems/count-servers-that-communicate/

from typing import List
class Solution:
    def countServers(self, grid: List[List[int]]) -> int:
        ans = 0
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == 1:
                    if sum(grid[i]) >= 2 or sum(grid[x][j] for x in range(len(grid))) >= 2:
                        ans += 1

        return ans
    
    
# GPT 优化
from typing import List

# 使用 comprehensions 来更简洁地计算行和列的和。
# 事先计算每行和每列的和，这样在循环中就不需要多次重复计算，从而提高效率。
# 使用更有意义的变量名以增强可读性。

class Solution:
    def countServers(self, grid: List[List[int]]) -> int:
        # 预计算每行和每列的服务器数量
        row_sums = [sum(row) for row in grid]
        col_sums = [sum(grid[i][j] for i in range(len(grid))) for j in range(len(grid[0]))]

        # 计算能够与其他服务器通信的服务器数量
        count = 0
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == 1 and (row_sums[i] > 1 or col_sums[j] > 1):
                    count += 1

        return count