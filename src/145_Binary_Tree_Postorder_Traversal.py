# 145_Binary_Tree_Postorder_Traversal
# https://leetcode.cn/problems/binary-tree-postorder-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        result = []

        def traversal(root):
            if root is None: return
            
            traversal(root.left)
            traversal(root.right)
            
            result.append(root.val)

        traversal(root)
        
        return result


class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []
        
        res = []
        stack = [root]
        
        while stack:
            node = stack.pop()
            
            if node.left:
                stack.append(node.left)
            if node.right:
                stack.append(node.right)
                
            res.append(node.val)

        res = res[::-1]

        return res
    
    

