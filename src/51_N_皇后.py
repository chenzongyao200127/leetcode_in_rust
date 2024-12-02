from typing import List

# 按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。
# n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
# 给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。
# 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        def is_valid(board, row, col):
            for i in range(row):
                if board[i] == col or abs(row - i) == abs(col - board[i]):
                    return False
            return True

        def backtrack(board, row):
            if row == n:
                res.append(
                    [''.join(['Q' if j == c else '.' for j in range(n)]) for c in board])
                return

            for col in range(n):
                if not is_valid(board, row, col):
                    continue
                board[row] = col
                backtrack(board, row + 1)
                board[row] = -1

        res = []
        board = [-1 for _ in range(n)]
        backtrack(board, 0)
        return res


# n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。
# 给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
class Solution:
    def totalNQueens(self, n: int) -> int:
        def is_valid(board, row, col):
            for i in range(row):
                if board[i] == col or abs(row - i) == abs(col - board[i]):
                    return False
                return True

        def dfs(board, row):
            if row == n:
                self.count += 1
                return

            for col in range(n):
                if not is_valid(board, row, col):
                    continue
            board[row] = col
            dfs(board, row + 1)
            board[row] = -1

        self.count = 0
        board = [-1 for _ in range(n)]
        dfs(board, 0)
        return self.count
