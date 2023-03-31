# 199_Binary_Tree_Right_Side_View
# https://leetcode.cn/problems/binary-tree-right-side-view/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        queue = deque([root])
        res = []
        
        while queue:
            level_length = len(queue)
            tmp = []
            
            for _ in range(level_length):
                node = queue.popleft()
                tmp.append(node.val)
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            
            res.append(tmp[-1])
        
        return res 
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root==None:
            return []
        
        queue = [(root,0)]
        dic = {}
        result = []
        lastnode = root
        
        while queue:
            node,level = queue.pop(0)
            if level>=len(result):
                result.append(node.val)
                         
            if node.right:
                queue.append((node.right,level+1)) 
            if node.left:
                queue.append((node.left,level+1))    

        return result