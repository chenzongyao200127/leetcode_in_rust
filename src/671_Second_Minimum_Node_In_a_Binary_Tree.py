# 671_Second_Minimum_Node_In_a_Binary_Tree
# https://leetcode.cn/problems/second-minimum-node-in-a-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def findSecondMinimumValue(self, root: Optional[TreeNode]) -> int:
            def dfs(node: TreeNode):
                if not node:
                    return -1

                if node.val > root_val:
                    return node.val

                left_val = dfs(node.left)
                right_val = dfs(node.right)

                if left_val == -1:
                    return right_val
                if right_val == -1:
                    return left_val

                return min(left_val, right_val)

            if not root:
                return -1

            root_val = root.val
            return dfs(root)

        
class Solution:
    def findSecondMinimumValue(self, root: Optional[TreeNode]) -> int:
        res = []
        def dfs(node: TreeNode):
            if node is None:
                return
            
            res.append(node.val)
            dfs(node.left)
            dfs(node.right)
            
        dfs(root)
        res = list(set(res))
        if len(res) < 2:
            return -1
        else:
            res.sort()
            return res[1]
