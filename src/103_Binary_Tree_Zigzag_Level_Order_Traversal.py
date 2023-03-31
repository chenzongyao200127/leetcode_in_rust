# 103_Binary_Tree_Zigzag_Level_Order_Traversal
# https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
    
class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        
        queue = deque([root])
        res = []
        flag = True
        
        while queue:
            level_length = len(queue)
            tmp = []
            
            for _ in range(level_length):
                node = queue.popleft()
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                    
                tmp.append(node.val)
            
            if flag is False:
                tmp = tmp[::-1]
            
            res.append(tmp)
            flag = not flag
            
        return res
        
        
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if not root:
            return []
        ans=[]
        stack=[root]
        need_change=False
        while stack:
            size=len(stack)
            res=[]
            new_stack=[]
            for _ in range(size):
                node=stack.pop()
                res.append(node.val)
                if need_change:
                    node.left,node.right=node.right,node.left
                if node.left:
                    new_stack.append(node.left)
                if node.right:
                    new_stack.append(node.right)
            stack=new_stack
            ans.append(res)
            need_change=not need_change
        return ans
        