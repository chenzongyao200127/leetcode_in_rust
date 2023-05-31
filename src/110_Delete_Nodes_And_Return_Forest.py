# 110_Delete_Nodes_And_Return_Forest
# https://leetcode.cn/problems/delete-nodes-and-return-forest/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
        
class Solution:
    def delNodes(self, root: Optional[TreeNode], to_delete: List[int]) -> List[TreeNode]:
        to_delete_set = set(to_delete)
        roots = []
        self.dfs(root, True, to_delete_set, roots)
        return roots
    
    def dfs(self, node: Optional[TreeNode], is_root: bool, to_delete_set: Set[int], roots: List[TreeNode]) ->  Optional[TreeNode]:
        if node == None:
            return None
        
        delete = node.val in to_delete_set
        node.left = self.dfs(node.left, delete, to_delete_set, roots)
        node.right = self.dfs(node.right, delete, to_delete_set, roots)
        
        if delete:
            return None
        else:
            if is_root:
                roots.append(node)
            return node