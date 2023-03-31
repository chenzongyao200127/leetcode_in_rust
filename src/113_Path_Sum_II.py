# 113_Path_Sum_II
# https://leetcode.cn/problems/path-sum-ii/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        if root is None:
            return []
        
        ans = []
        path = []
        
        def dfs(root: TreeNode, targetSum: int):
            if not root:
                return
            
            path.append(root.val)
            
            targetSum -= root.val
            
            if not root.left and not root.right and targetSum == 0:
                ans.append(path[:])
                
            dfs(root.left, targetSum)
            dfs(root.right, targetSum)
            
            path.pop()

        dfs(root, targetSum)
        return ans
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        res = []
        if not root:
            return res
        path = []

        def dfs(root, target):
            if not root.left and not root.right and target == targetSum:
                res.append(path[:])
                
            if not root.left and not root.right:
                return
            
            if root.left:
                path.append(root.left.val)
                dfs(root.left, target + root.left.val)
                path.pop()
                
            if root.right:
                path.append(root.right.val)
                dfs(root.right, target + root.right.val)
                path.pop()
                
        path.append(root.val)
        
        dfs(root, root.val)
        return res
    

class Solution:
    def pathSum(self, root: TreeNode, targetSum: int) -> List[List[int]]:
        ret = list()
        parent = collections.defaultdict(lambda: None)

        def getPath(node: TreeNode):
            tmp = list()
            while node:
                tmp.append(node.val)
                node = parent[node]
            ret.append(tmp[::-1])

        if not root:
            return ret
        
        que_node = collections.deque([root])
        que_total = collections.deque([0])

        while que_node:
            node = que_node.popleft()
            rec = que_total.popleft() + node.val

            if not node.left and not node.right:
                if rec == targetSum:
                    getPath(node)
            else:
                if node.left:
                    parent[node.left] = node
                    que_node.append(node.left)
                    que_total.append(rec)
                if node.right:
                    parent[node.right] = node
                    que_node.append(node.right)
                    que_total.append(rec)

        return ret