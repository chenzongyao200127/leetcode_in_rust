# 124_Binary_Tree_Maximum_Path_Sum
# https://leetcode.cn/problems/binary-tree-maximum-path-sum/

# 二叉树中的 路径 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
# 同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个 节点，且不一定经过根节点。

# 路径和 是路径中各节点值的总和。

# 给你一个二叉树的根节点 root ，返回其 最大路径和 。


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
from typing import Optional
        
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        ans = float('-inf')
        
        def dfs(node):
            if not node:
                return float('-inf'), float('-inf')
            
            l_with_root, l_without_root = dfs(node.left)
            r_with_root, r_without_root = dfs(node.right)
            
            with_root = max(
                node.val,
                node.val + l_with_root,
                node.val + r_with_root,
                node.val + l_with_root + r_with_root
            )

            return_with_root = max(
                node.val,
                node.val + l_with_root,
                node.val + r_with_root,
            )
            
            without_root = max(l_without_root, l_with_root, r_without_root, r_with_root)

            nonlocal ans
            ans = max(ans, with_root, without_root)
            
            return return_with_root, without_root
        
        dfs(root)
        return ans
    
    
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        self.result = float('-inf')
        
        def dfs(node):
            if not node:
                return 0
            
            # Maximum value we can get if we choose to go through left child
            left = max(dfs(node.left), 0)
            
            # Maximum value we can get if we choose to go through right child
            right = max(dfs(node.right), 0)
            
            # Maximum path value if we choose node as the highest node in this path
            # This is not necessarily a path from leaf to leaf
            self.result = max(self.result, left + right + node.val)
            
            # Maximum value if we want to extend the path to parent
            return max(left, right) + node.val
        
        dfs(root)
        return self.result
