# 529_Minesweeper
# https://leetcode.cn/problems/minesweeper/

# 让我们一起来玩扫雷游戏！

# 给你一个大小为 m x n 二维字符矩阵 board ，表示扫雷游戏的盘面，其中：

# 'M' 代表一个 未挖出的 地雷，
# 'E' 代表一个 未挖出的 空方块，
# 'B' 代表没有相邻（上，下，左，右，和所有4个对角线）地雷的 已挖出的 空白方块，
# 数字（'1' 到 '8'）表示有多少地雷与这块 已挖出的 方块相邻，
# 'X' 则表示一个 已挖出的 地雷。
# 给你一个整数数组 click ，其中 click = [clickr, clickc] 表示在所有 未挖出的 方块（'M' 或者 'E'）中的下一个点击位置（clickr 是行下标，clickc 是列下标）。

# 根据以下规则，返回相应位置被点击后对应的盘面：

# 如果一个地雷（'M'）被挖出，游戏就结束了- 把它改为 'X' 。
# 如果一个 没有相邻地雷 的空方块（'E'）被挖出，修改它为（'B'），并且所有和其相邻的 未挖出 方块都应该被递归地揭露。
# 如果一个 至少与一个地雷相邻 的空方块（'E'）被挖出，修改它为数字（'1' 到 '8' ），表示相邻地雷的数量。
# 如果在此次点击中，若无更多方块可被揭露，则返回盘面。


from typing import List

class Solution:
    def updateBoard(self, board: List[List[str]], click: List[int]) -> List[List[str]]:
        m = len(board)
        n = len(board[0])
        click_x = click[0]
        click_y = click[1]
        
        def dfs(x, y, board):
            cnt = 0
            for i in range(-1, 2):
                for j in range(-1, 2):
                    if 0 <= x + i < m and 0 <= y + j < n and board[x+i][y+j] == "M":
                        cnt += 1
            if cnt == 0:
                board[x][y] = "B"
                for i in range(-1, 2):
                    for j in range(-1, 2):
                        if 0 <= x + i < m and 0 <= y + j < n and board[x+i][y+j] == "E":
                            dfs(x+i, y+j, board) # Fixed error here
            else:
                board[x][y] = str(cnt)
                return
        
        if board[click_x][click_y] == "M":
            board[click_x][click_y] = "X"
            return board
        else:
            dfs(click_x, click_y, board)
        
        return board

# 代码优化
class Solution:
    def updateBoard(self, board: List[List[str]], click: List[int]) -> List[List[str]]:
        m, n = len(board), len(board[0])
        x, y = click

        directions = [(i, j) for i in range(-1, 2) for j in range(-1, 2) if (i, j) != (0, 0)]
        
        def dfs(x, y):
            if not(0 <= x < m) or not(0 <= y < n) or board[x][y] != "E":
                return
            cnt = sum(board[x+dx][y+dy] == "M" for dx, dy in directions if 0 <= x+dx < m and 0 <= y+dy < n)
            if cnt == 0:
                board[x][y] = "B"
                for dx, dy in directions:
                    dfs(x+dx, y+dy)
            else:
                board[x][y] = str(cnt)
        
        if board[x][y] == "M":
            board[x][y] = "X"
        else:
            dfs(x, y)
        
        return board
