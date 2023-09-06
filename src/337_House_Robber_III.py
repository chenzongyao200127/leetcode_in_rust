# 337_House_Robber_III
# https://leetcode.cn/problems/house-robber-iii/description/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
from typing import Optional
class Solution:
    def rob(self, root: Optional[TreeNode]) -> int:
        def dfs(node):
            if not node:
                return 0, 0
            
            l_rob, l_not_rob = dfs(node.left)
            r_rob, r_not_rob = dfs(node.right)
            
            rob = l_not_rob + r_not_rob + node.val
            not_rob = max(l_not_rob, l_rob) + max(r_not_rob, r_rob)
            
            return rob, not_rob
        
        return max(dfs(root))