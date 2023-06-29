# 79_Word_Search
# https://leetcode.cn/problems/word-search/


# 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。
# 如果 word 存在于网格中，返回 true ；否则，返回 false 。
# 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。
# 同一个单元格内的字母不允许被重复使用。

# 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
# 输出：true

# 进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？
class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        dirs = [[1,0], [0, 1], [-1, 0], [0, -1]]

        m = len(board)
        n = len(board[0])
        
        def dfs(x, y, board, word, idx, visited):
            if idx == len(word):
                return True

            visited[y][x] = True
            for dir in dirs:
                next_x = x + dir[0]
                next_y = y + dir[1]
                if 0 <= next_x < n and 0 <= next_y < m \
                    and not visited[next_y][next_x] and board[next_y][next_x] == word[idx]:
                    if dfs(next_x, next_y, board, word, idx + 1, visited):
                        return True
            visited[y][x] = False
            return False
                
        
        visited = [[False for _ in range(n)] for _ in range(m)]
        for i in range(m):
            for j in range(n):
                if board[i][j] == word[0]:
                    if dfs(j, i, board, word, 1, visited):
                        return True

        return False
                
                
class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        # 定义一个递归辅助函数来执行深度优先搜索（DFS）
        def step(board, cur, history, word, offset):
            # 基本情况：如果偏移量到达单词的结尾，返回True
            if offset == len(word):
                return True

            x, y = cur
            candidates = set()

            # 如果在网格边界内，将可能的下一个位置添加到候选集中
            if x > 0:
                candidates.add((x - 1, y))
            if x < len(board) - 1:
                candidates.add((x + 1, y))
            if y > 0:
                candidates.add((x, y - 1))
            if y < len(board[0]) - 1:
                candidates.add((x, y + 1))

            # 从候选集中移除已经访问过的位置（history）
            # 在这个深度优先搜索（DFS）算法中，candidates 集合表示当前位置 (x, y) 的四个相邻位置（上、下、左、右），
            # 这些位置是潜在的搜索路径。
            # 然而，在搜索过程中，我们需要避免重复访问同一个位置，因为这会导致无限循环或者错误的路径。
            # 为了避免这种情况，我们需要跟踪已经访问过的位置，这就是 history 集合的作用。
            # 我们从 candidates 集合中移除了已经在 history 集合中的位置。
            # 这样，final 集合就只包含了尚未访问过的相邻位置。
            # 然后，算法会遍历 final 集合中的位置，根据 DFS 算法继续搜索。
            # 这一步确保了搜索过程中不会重复访问同一个位置，从而避免了无限循环和错误路径。
            final = candidates - history

            # 如果没有候选位置可供探索，返回False
            if not len(final):
                return False

            # 遍历剩余的候选位置
            for p in final:
                x2, y2 = p
                # 如果候选位置的字符与当前单词字符匹配，继续DFS
                if board[x2][y2] == word[offset]:
                    history.add(p)
                    if step(board, p, history, word, offset + 1):
                        return True
                    history.remove(p)

        # 检查网格中起始字符和结束字符的数量
        x = 0
        for i in range(len(board)):
            for j in range(len(board[0])):
                if board[i][j] == word[0]:
                    x += 1
                elif board[i][j] == word[-1]:
                    x -= 1
        if x > 0:
            word = word[::-1]

        height = len(board)
        width = len(board[0])

        # 遍历网格中的所有单元格
        for i in range(height):
            for j in range(width):
                # 如果当前单元格的字符与单词的第一个字符匹配，开始DFS
                if board[i][j] == word[0]:
                    p1 = (i, j)
                    history = {p1,}
                    if step(board, p1, history, word, 1):
                        return True

        # 如果找不到路径，返回False
        return False