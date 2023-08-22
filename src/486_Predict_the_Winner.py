# 486_Predict_the_Winner
# https://leetcode.cn/problems/predict-the-winner/


# The problem you are trying to solve is the "Predict the Winner" problem 
# where two players play optimally to maximize their score by choosing from the ends of an array of numbers.

# There are some issues with your approach:

# The @cache decorator should come from the functools module, which you need to import.

# The main logic for checking who wins is not correct, 
# and the implementation doesn't capture the essence of dynamic programming or memoization.

# The use of AB array and A & B variables is quite confusing and redundant.

# Let's simplify and correct the code:
from typing import List
class Solution:
    def predictTheWinner(self, nums: List[int]) -> bool:
        AB = [0] * 2
        A = 0
        B = 0
        def dfs(cur_nums, turn):
            if cur_nums == []:
                A = max(A, AB[0])
                B = max(B, AB[1])   
                
            f = cur_nums[0]
            AB[turn] += f
            dfs(cur_nums[1:], (turn + 3 ) // 2)
            AB[turn] -= f
            
            l = cur_nums[-1]
            AB[turn] += l
            dfs(cur_nums[:-2], (turn + 3 ) // 2)
            AB[turn] -= l
            
        return A >= B
    
# Here's what's happening:

# The function dfs receives two pointers i and j that represent a subarray of nums. 
# The parameter turn determines whose turn it is. 1 means it's player 1's turn and -1 means it's player 2's turn.

# If i == j, there's only one number left, so the current player picks that number.

# Otherwise, the player has two choices: either pick the first or the last number 
# of the current subarray. We compute the resulting score for both choices and take the best one.

# Since both players play optimally, each player tries to maximize their own score. 
# This is modeled by multiplying the score of the current player's turn by turn and taking the maximum.

# The main function then calls dfs for the whole array and checks 
# if player 1's score is non-negative (indicating a win or a tie for player 1). 
# If it's non-negative, then player 1 can either win or tie, so we return True. 
# If it's negative, then player 2 wins, and we return False.

from typing import List
from functools import lru_cache

class Solution:
    def predictTheWinner(self, nums: List[int]) -> bool:
        @lru_cache(None)
        def dfs(i, j, turn):
            if i == j:
                return turn * nums[i]
            
            pickFirst = turn * nums[i] + dfs(i + 1, j, -turn)
            pickLast = turn * nums[j] + dfs(i, j - 1, -turn)
            
            return max(pickFirst * turn, pickLast * turn) * turn
        
        return dfs(0, len(nums) - 1, 1) >= 0


# 递归优化
from typing import List
from functools import lru_cache

class Solution:
    def predictTheWinner(self, nums: List[int]) -> bool:
        @lru_cache(None)
        def dfs(i, j):
            if i == j:
                return nums[i]
            
            pickFirst = nums[i] - dfs(i + 1, j)
            pickLast = nums[j] - dfs(i, j - 1)
            
            return max(pickFirst, pickLast)
        
        return dfs(0, len(nums) - 1) >= 0
