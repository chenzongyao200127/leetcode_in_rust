# 1448_Count_Good_Nodes_in_Binary_Tree
# https://leetcode.cn/problems/count-good-nodes-in-binary-tree/

# Given a binary tree root, a node X in the tree is named good if in the path from root to X 
# there are no nodes with a value greater than X.
# Return the number of good nodes in the binary tree.

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        t = -10001
        cnt = 0
        def dfs(node, t):
            if not node:
                return
            
            if node.val >= t:
                nonlocal cnt
                cnt += 1
            
            new_t = max(t, node.val)
            dfs(node.left, new_t)
            dfs(node.right, new_t)

        dfs(root, t)
        
        return cnt


# 简洁写法
class Solution:
    def goodNodes(self, root: TreeNode, mx=-inf) -> int:
        if root is None:
            return 0
        left = self.goodNodes(root.left, max(mx, root.val))
        right = self.goodNodes(root.right, max(mx, root.val))
        return left + right + (mx <= root.val)
