# 450_Delete_Node_in_a_BST
# https://leetcode.cn/problems/delete-node-in-a-bst/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:            
        if root is None:
            return None
    
        if key < root.val:
            root.left = self.deleteNode(root.left, key)
            return root
        
        elif key > root.val:
            root.right = self.deleteNode(root.right, key)
            return root
        
        else:
            if not root.left and not root.right:
                return None
            
            elif not root.left or not root.right:
                return root.left or root.right
            
            else:
                # 找到右子树中的最小节点
                min_node = root.right
                
                while min_node.left:
                    min_node = min_node.left
                    
                # 将最小节点的值赋给当前节点
                root.val = min_node.val
                
                # 在右子树中删除最小节点
                root.right = self.deleteNode(root.right, min_node.val)
                
        return root

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if not root:    return None
        
        if root.val < key:
            root.right = self.deleteNode(root.right, key)
            
        elif root.val > key:
            root.left = self.deleteNode(root.left, key)
            
        else:
            # case 1: right tree exist but left tree not
            if not root.left and root.right:    return root.right
            # case 2: left tree exist but right tree not
            elif not root.right and root.left:    return root.left
            # case 3: left tree exist and right tree exist
            elif root.left and root.right:
                node = root.right
                while node.left:
                    node = node.left
                node.left = root.left
                return root.right
            # case 4: left tree and right tree are not exist
            else:
                return None

        return root