# 526_Beautiful_Arrangement
# https://leetcode.cn/problems/beautiful-arrangement/solution/you-mei-de-pai-lie-by-leetcode-solution-vea2/

# 我们可以使用回溯法解决本题，从左向右依次向目标排列中放入数即可。
# 特别地，为了优化回溯效率，我们可以预处理每个位置的符合条件的数有哪些
class Solution:
    def countArrangement(self, n: int) -> int:
        match = defaultdict(list)
        for i in range(1, n+1):
            for j in range(1, n+1):
                if i % j == 0 or j % i == 0:
                    match[i].append(j)
                    
        num = 0
        vis = set()
        
        def backtrace(index: int) -> None:
            if index == n + 1:
                nonlocal num
                num += 1
                return
            
            for x in match[index]:
                if x not in vis:
                    vis.add(x)
                    backtrace(index+1)
                    vis.discard(x)
                    
        backtrace(1)
        return num