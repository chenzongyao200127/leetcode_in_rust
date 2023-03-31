# 589_N-ary_Tree_Preorder_Traversal
# https://leetcode.cn/problems/n-ary-tree-preorder-traversal/

"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        if root is None:
            return []
        
        stack = [root]
        res = []
        
        while stack:
            node = stack.pop()
            res.append(node.val)
            
            for child in node.children[::-1]:
                stack.append(child)
        
        return res
        
# 递归
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        ans = []
        
        def dfs(node: 'Node'):
            if node is None:
                return
            
            ans.append(node.val)
            
            for ch in node.children:
                dfs(ch)
                
        dfs(root)
        return ans
