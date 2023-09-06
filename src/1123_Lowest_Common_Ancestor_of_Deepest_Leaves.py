# 1123_Lowest_Common_Ancestor_of_Deepest_Leaves
# https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/

# 给你一个有根节点 root 的二叉树，返回它 最深的叶节点的最近公共祖先 。

# 回想一下：

# 叶节点 是二叉树中没有子节点的节点

# 树的根节点的 深度 为 0，如果某一节点的深度为 d，那它的子节点的深度就是 d+1

# 如果我们假定 A 是一组节点 S 的 最近公共祖先
# S 中的每个节点都在以 A 为根节点的子树中，且 A 的深度达到此条件下可能的最大值。

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
from typing import Optional        

class Solution:
    def lcaDeepestLeaves(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(node):
            if not node:
                return 0, None
            
            l_depth, l_subtree = dfs(node.left)
            r_depth, r_subtree = dfs(node.right)
            
            if l_depth == r_depth:
                return l_depth+1, node
            elif l_depth > r_depth:
                return l_depth+1, l_subtree
            else:
                return r_depth+1, r_subtree
            
        return dfs(root)[1]