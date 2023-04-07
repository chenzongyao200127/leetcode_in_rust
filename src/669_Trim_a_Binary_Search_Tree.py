# 669_Trim_a_Binary_Search_Tree
# https://leetcode.cn/problems/trim-a-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def trimBST(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        if root is None:
            return None
        
        if root.val < low:
            return self.trimBST(root.right, low, high)
        
        if root.val > high:
            return self.trimBST(root.left, low, high)
        
        root.left = self.trimBST(root.left, low, high)
        root.right = self.trimBST(root.right, low, high)
        
        return root


class Solution:
    def trimBST(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        if(root is None):
            return None
        
        if(low <= root.val <= high):
            root.left = self.trimBST(root.left, low, high)
            root.right = self.trimBST(root.right, low, high)
            
        else:
            while(root and (root.val<low or root.val>high)):
                while(root and root.val < low):
                    root = root.right
                while(root and root.val > high):
                    root = root.left
            return self.trimBST(root, low, high)
        
        return root



class Solution:
    def trimBST(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        # 我们对根结点进行判断，如果根结点不符合要求，我们将根结点设为对应的左结点或右结点，
        # 直到根结点符合要求，然后将根结点作为符合要求的结点，依次修剪它的左子树与右子树。
        while root and (root.val < low or root.val > high):
            root = root.right if root.val < low else root.left
            
        if root is None:
            return None
        
        node = root
        
        # 如果根节点的值小于low，将根节点更新为其右子节点；否则，将根节点更新为其左子节点。
        while node.left:
            if node.left.val < low:
                node.left = node.left.right
            else:
                node = node.left
        
        node = root
        
        # 这个while循环用于修剪左子树。如果node.left存在且其值小于low，
        # 则将node.left更新为node.left.right；否则，将node更新为node.left。
        while node.right:
            if node.right.val > high:
                node.right = node.right.left
            else:
                node = node.right
        
        return root