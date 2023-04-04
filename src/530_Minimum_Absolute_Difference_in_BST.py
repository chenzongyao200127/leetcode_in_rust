# 530_Minimum_Absolute_Difference_in_BST
# https://leetcode.cn/problems/minimum-absolute-difference-in-bst/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
        
# 这段代码的问题在于它只计算了根节点与其直接相邻的左右子节点之间的差值，
# 并没有考虑到二叉搜索树的性质。
# 在二叉搜索树中，最小绝对差值是相邻节点之间的差值，其中相邻节点是指中序遍历时的相邻节点。
# 要修复这个问题，可以通过修改代码以实现中序遍历，并在遍历过程中计算相邻节点之间的差值。

# class Solution:
#     def getMinimumDifference(self, root: Optional[TreeNode]) -> int:
#         ans = 10001
        
#         def dfs(node: TreeNode):
#             if node is None:
#                 return False
            
#             nonlocal ans
            
#             if node.left:
#                 ans = min(ans, abs(node.val - node.left.val))
#             if node.right:
#                 ans = min(ans, abs(node.val - node.right.val))
            
#             dfs(node.left)
#             dfs(node.right)
            
#         dfs(root)
        
#         return ans

class Solution:
    def getMinimumDifference(self, root: Optional[TreeNode]) -> int:
        ans = float("inf")
        prev = None
        
        def inorder(node: TreeNode):
            nonlocal ans, prev
            
            if node is None:
                return
            
            inorder(node.left)
            
            if prev is not None:
                ans = min(ans, abs(node.val - prev.val))
            prev = node
            
            inorder(node.right)
            
        inorder(root)
        
        return ans
                
        