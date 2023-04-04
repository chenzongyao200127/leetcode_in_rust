# 230_Kth_Smallest_Element_in_a_BST
# https://leetcode.cn/problems/kth-smallest-element-in-a-bst/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        order = []
        
        def dfs(node: TreeNode):
            if node is None:
                return node
            
            dfs(node.left)
            
            nonlocal order
            order.append(node.val)
            
            dfs(node.right)
            
        dfs(root)
        return order[k-1]
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        stack=[]
        
        while root or stack:
            
            while root:
                stack.append(root)
                root=root.left
                
            root=stack.pop()
            
            k-=1
            
            if k==0:
                return root.val
            
            root=root.right
