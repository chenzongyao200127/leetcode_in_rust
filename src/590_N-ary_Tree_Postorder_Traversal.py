# 590_N-ary_Tree_Postorder_Traversal
# https://leetcode.cn/problems/n-ary-tree-postorder-traversal/


# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        res = []
        
        def dfs(node: 'Node'):
            if node is None:
                return
            
            for child in node.children:
                dfs(child)
                
            res.append(node.val)
            
        dfs(root)
        
        return res
    
    
"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        res = []
        def postor(root):
            if root:
                for c in root.children:
                    postor(c)
                res.append(root.val)
        postor(root)
        return res