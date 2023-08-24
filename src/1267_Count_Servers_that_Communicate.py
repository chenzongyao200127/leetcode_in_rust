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