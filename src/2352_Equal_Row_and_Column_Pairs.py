# 2352_Equal_Row_and_Column_Pairs
# https://leetcode.cn/problems/equal-row-and-column-pairs/submissions/

# 暴力
class Solution:
    def equalPairs(self, grid: List[List[int]]) -> int:
        def is_equal(row, col):
            for i in range(len(row)):
                if row[i] != col[i]:
                    return False
            return True

        n = len(grid)
        count = 0
        for r in range(n):
            for c in range(n):
                if is_equal(grid[r], [grid[i][c] for i in range(n)]):
                    count += 1
        return count
    
    
class Solution:
    def equalPairs(self, grid: List[List[int]]) -> int:
        n = len(grid)
        cnt = Counter(tuple(r) for r in grid)
        ans = 0
        for col in zip(*grid):
            ans += cnt[col]
        return ans