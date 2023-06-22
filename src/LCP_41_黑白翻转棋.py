# LCP_41_黑白翻转棋
# https://leetcode.cn/problems/fHi6rV/

# BFS 写的很牛逼，要仔细学习 
class Solution:
    # 定义一个翻转棋盘的方法, 参数为棋盘字符串
    def flipChess(self, chessboard: str) -> int:
        # 定义8个方向
        dirs = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1))
        # 获取棋盘的行数和列数
        m, n = len(chessboard), len(chessboard[0])

        # 定义一个搜索方法，用于翻转棋子
        def search(board, i, j):
            # 将当前位置设置为 'X'
            board[i][j] = "X"
            # 初始化翻转计数为0
            ans = 0
            # 遍历8个方向
            for di, dj in dirs:
                # 初始化一个临时列表，用于存储需要翻转的棋子位置
                tmp = []
                # 初始化新的坐标
                ni, nj = i, j
                # 沿着当前方向进行搜索
                while 0 <= (ni := ni + di) < m and 0 <= (nj := nj + dj) < n:
                    # 如果新位置是 'X' 或者 '.'，则停止搜索
                    if board[ni][nj] in "X.":
                        # 如果新位置是 'X'，则翻转临时列表中的棋子，并更新翻转计数
                        if board[ni][nj] == "X":
                            for ti, tj in tmp:
                                board[ti][tj] = "X"
                            ans += len(tmp) + sum(search(board, ti, tj) for ti, tj in tmp)
                        break
                    # 如果新位置是 'O'，则将其添加到临时列表
                    tmp.append((ni, nj))
            # 返回翻转计数
            return ans

        # 遍历棋盘上的每个空位置，计算翻转后的最大得分
        return max((search(list(map(list, chessboard)), i, j) for i in range(m) for j in range(n) if chessboard[i][j] == "."), default=0)
    
    
    
# := 运算符在 Python 中被称为“海象运算符”（walrus operator），它是在 Python 3.8 版本中引入的新特性。
# 其作用是在表达式内部为变量赋值。这样做有时可以使代码更简洁，因为你可以在一个表达式中同时计算和赋值。
# 在给定的代码中：
# while 0 <= (ni := ni + di) < m and 0 <= (nj := nj + dj) < n:
# := 运算符用于在循环内同时更新 ni 和 nj 的值。
# 这里，ni 的新值等于 ni + di，nj 的新值等于 nj + dj。
# 使用海象运算符可以让我们在条件语句中同时更新这两个变量，而不需要在循环体中单独更新它们。

class Solution:
    def flipChess(self, chessboard: List[str]) -> int:
        def judge(chessboard: List[List[str]], x: int, y: int, dx: int, dy: int) -> bool:
            x += dx
            y += dy
            while 0 <= x < len(chessboard) and 0 <= y < len(chessboard[0]):
                if chessboard[x][y] == "X":
                    return True
                elif chessboard[x][y] == ".":
                    return False
                x += dx
                y += dy
            return False
        
        def bfs(chessboard: List[str], px: int, py: int) -> int:
            chessboard = [list(row) for row in chessboard]
            cnt = 0
            q = deque([(px, py)])
            chessboard[px][py] = "X"

            while q:
                tx, ty = q.popleft()
                for dx in [-1, 0, 1]:
                    for dy in [-1, 0, 1]:
                        if dx == dy == 0:
                            continue
                        if judge(chessboard, tx, ty, dx, dy):
                            x, y = tx + dx, ty + dy
                            while chessboard[x][y] != "X":
                                q.append((x, y))
                                chessboard[x][y] = "X"
                                x += dx
                                y += dy
                                cnt += 1
            return cnt

        res = 0
        for i in range(len(chessboard)):
            for j in range(len(chessboard[0])):
                if chessboard[i][j] == ".":
                    res = max(res, bfs(chessboard, i, j))
        return res
