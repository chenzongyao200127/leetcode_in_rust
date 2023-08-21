# 2337. Move Pieces to Obtain a String
# https://leetcode.cn/problems/move-pieces-to-obtain-a-string/description/


# The condition `if i != j and (c == 'L') == (i < j):` contains two parts, 
# and understanding both is crucial for grasping the logic of the algorithm:

# 1. `i != j`: This checks whether the current character from the `start` 
# string and the corresponding character from the `target` string are in different positions. 
# If `i == j`, it means the current character hasn't moved, 
# so there's no need to check any further for this character since it's already in the correct position in the target string. 

# 2. `(c == 'L') == (i < j)`: This condition is used to determine the legality of the move. 
# If the character is 'L', it means it can only move to the left. Hence, its starting position (`i`) 
# should be greater than its ending position (`j`). If the character is 'R', it can only move to the right, 
# so its starting position should be less than its ending position. Thus, the condition checks for the legality 
# of the move based on the character and its positions in both strings.

# Putting the two conditions together:

# - If `i == j`, the character hasn't moved, and we continue with the next character.
# - If `i != j`, the character has moved, and then we need to check if the move was legal. The legality is checked by the second part of the condition.

# So, `i != j` acts as a filter to only inspect moves where the character has actually changed its position. If we didn't include this condition, it would be possible to flag legal non-moves (characters that haven't moved) as illegal.
class Solution:
    def canChange(self, start: str, target: str) -> bool:
        if start.replace('_', '') != target.replace('_', ''):
            return False

        j = 0
        for i, c in enumerate(start):
            if c == '_':
                continue
            
            while target[j] == '_':
                j += 1
            
            if i != j and (c == 'L') == (i < j):
                return False
            
            j += 1
            
        return True
            