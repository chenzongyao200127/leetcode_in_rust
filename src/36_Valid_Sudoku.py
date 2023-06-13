# 36_Valid_Sudoku
# https://leetcode.cn/problems/valid-sudoku/

class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row = [[False] * 9 for _ in range(9)]
        col = [[False] * 9 for _ in range(9)]
        block = [[False] * 9 for _ in range(9)]

        for i in range(9):
            for j in range(9):
                if board[i][j] != '.':
                    num = int(board[i][j]) - 1
                    blockIndex = (i // 3) * 3 + j // 3
                    if row[i][num] or col[j][num] or block[blockIndex][num]:
                        return False
                    else:
                        row[i][num] = True
                        col[j][num] = True
                        block[blockIndex][num] = True

        return True

# 列表推导式（List Comprehension）是 Python 中一种非常简洁且优雅的创建列表的方式。
# 它允许你在一行代码中生成新列表，而不需要使用多行循环和条件语句。

# 列表推导式的基本形式如下：
# [expression for item in iterable if condition]
# 这里的 expression 是用于计算新列表中的每个元素的表达式，
# item 是从 iterable 中遍历的每个元素，而 condition 是一个可选的过滤条件，只有满足条件的元素才会被添加到新列表中。

# 让我们通过一些示例来更好地理解列表推导式：
# 1. 生成一个包含 0 到 9 的平方的列表：
squares = [x ** 2 for x in range(10)]
# 输出: [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]

# 从一个列表中筛选出所有的偶数：
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9]
even_numbers = [x for x in numbers if x % 2 == 0]
# 输出: [2, 4, 6, 8]

# 使用嵌套列表推导式创建一个乘法表：
multiplication_table = [[x * y for x in range(1, 11)] for y in range(1, 11)]
# 注意，虽然列表推导式使得代码更简洁，但在某些情况下，过度使用可能会导致代码可读性降低。
# 因此，请在适当的场合使用列表推导式，并保持代码的简洁和可读性。