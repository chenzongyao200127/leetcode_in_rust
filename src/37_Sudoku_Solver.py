# 37_Sudoku_Solver
# https://leetcode.cn/problems/sudoku-solver/

class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        def isValid(board, row, col, num):
            for i in range(9):
                if board[row][i] == num:
                    return False
                if board[i][col] == num:
                    return False
                if board[3 * (row // 3) + i // 3][3 * (col // 3) + i % 3] == num:
                    return False
            return True

        def backtrack(board):
            for i in range(9):
                for j in range(9):
                    if board[i][j] == '.':
                        for k in range(1, 10):
                            if isValid(board, i, j, str(k)):
                                board[i][j] = str(k)
                                if backtrack(board):
                                    return True
                                board[i][j] = '.'
                        return False
            return True

        backtrack(board)


class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        row = col = 9
        # 位计数行列块
        rowList = [0] * row 
        colList = [0] * col
        blockList = [[0] * (col // 3) for _ in range(row//3)] 
        blanks = []
        fin = False # 全局变量标识是否已填完
        
        # 通过i行j列的数字num进行位的翻转
        def updateLists(num, i, j):
            rowList[i] ^= (1 << (num-1))
            colList[j] ^= (1 << (num-1))
            blockList[i//3][j//3] ^= (1 << (num-1))

        # 找到讨论情况数最少的空块,返回坐标和可能填入的数字,坐标为-1表示无解，-2表示填完了
        def getMinInd():
            min_n, min_i, min_j, min_nums, min_ind = 10, -2, -2, 0, -1
            for ind, (i, j) in enumerate(blanks):
                # 获取可能填的数字（1表示可以填的数字）
                pos_nums = ~(rowList[i] | colList[j] | blockList[i//3][j//3]) & 0x1ff
                # 记录可能填的数字种数
                pos_n = bin(pos_nums).count("1")
                if pos_n == 0: # 无解了
                    return -1, -1, 0
                if pos_n == 1: # 唯一解
                    blanks.pop(ind)
                    return i, j, pos_nums
                if pos_n < min_n: # 有更小的情况
                    min_ind, min_n, min_nums, min_i, min_j = ind, pos_n, pos_nums, i, j
            if blanks:
                blanks.pop(min_ind)
            return min_i, min_j, min_nums

        # 尝试更新棋盘
        def dfs():
            nonlocal fin
            min_i, min_j, pos_nums = getMinInd()
            if min_i == -2:
                fin = True
                return
            if min_i == -1:
                return
            # 没有填完
            while pos_nums: # 尝试每个1的位
                digit = pos_nums & (-pos_nums) #获取最低位的1
                num = bin(digit).count("0")
                updateLists(num, min_i, min_j)
                board[min_i][min_j] = str(num)
                pos_nums &= (pos_nums - 1) # 去掉最低位的1
                dfs() # 尝试更新下一块
                if fin:
                    return 
                updateLists(num, min_i, min_j) # 撤回更新
            blanks.append((min_i, min_j)) # 重新加入该空格

        # 初始遍历，更新计数和空白块
        for i in range(row):
            for j in range(col):
                num = board[i][j]
                if num != ".":
                    updateLists(int(num), i, j)
                else:
                    blanks.append((i, j))
        dfs()
        return board   
    
    
class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        def flip(i: int, j: int, digit: int):
            line[i] ^= (1 << digit)
            column[j] ^= (1 << digit)
            block[i // 3][j // 3] ^= (1 << digit)

        def dfs(pos: int):
            nonlocal valid
            if pos == len(spaces):
                valid = True
                return
            
            i, j = spaces[pos]
            mask = ~(line[i] | column[j] | block[i // 3][j // 3]) & 0x1ff
            while mask:
                digitMask = mask & (-mask)
                digit = bin(digitMask).count("0") - 1
                flip(i, j, digit)
                board[i][j] = str(digit + 1)
                dfs(pos + 1)
                flip(i, j, digit)
                mask &= (mask - 1)
                if valid:
                    return
            
        line = [0] * 9
        column = [0] * 9
        block = [[0] * 3 for _ in range(3)]
        valid = False
        spaces = list()

        for i in range(9):
            for j in range(9):
                if board[i][j] != ".":
                    digit = int(board[i][j]) - 1
                    flip(i, j, digit)
        
        while True:
            modified = False
            for i in range(9):
                for j in range(9):
                    if board[i][j] == ".":
                        mask = ~(line[i] | column[j] | block[i // 3][j // 3]) & 0x1ff
                        if not (mask & (mask - 1)):
                            digit = bin(mask).count("0") - 1
                            flip(i, j, digit)
                            board[i][j] = str(digit + 1)
                            modified = True
            if not modified:
                break
        
        for i in range(9):
            for j in range(9):
                if board[i][j] == ".":
                    spaces.append((i, j))

        dfs(0)