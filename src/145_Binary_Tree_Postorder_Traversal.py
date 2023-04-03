# 145_Binary_Tree_Postorder_Traversal
# https://leetcode.cn/problems/binary-tree-postorder-traversal/

# Definition for a binary tree node.
# 前序
class Solution:
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        res = []
        
        def preorder(root: TreeNode):
            if not root:
                return
            
            res.append(root.val)
            
            preorder(root.left)
            preorder(root.right)

        preorder(root)
        
        return res
    
# 后序
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

        
# 后序
class Solution:
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return list()
        
        res = list()
        stack = list()
        prev = None

        while root or stack:
            while root:
                stack.append(root)
                root = root.left
            root = stack.pop()
            if not root.right or root.right == prev:
                res.append(root.val)
                prev = root
                root = None
            else:
                stack.append(root)
                root = root.right
        
        return res
    
    
# 前序
class Solution:
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:

        if not root:
            return []
            
        result = []

        stack = [root]

        while stack:
            cur_value = stack[-1]
            stack = stack[:-1]
            result += [cur_value.val]

            if cur_value.right:
                stack += [cur_value.right]
            if cur_value.left:
                stack += [cur_value.left]
        
        return result

            

