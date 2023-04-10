# 105_Construct_Binary_Tree_from_Preorder_and_Inorder_Traversal
# https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder or not inorder:
            return None
        
        root = TreeNode(preorder[0])
        idx = inorder.index(root.val)
        
        left_inorder = inorder[:idx]
        right_inorder = inorder[idx+1:]
        
        left_preorder = preorder[1:1+len(left_inorder)]
        right_preorder = preorder[1+len(left_preorder):]
        
        root.left = self.buildTree(left_preorder, left_inorder)
        root.right = self.buildTree(right_preorder,right_inorder)
        
        return root
        
        
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
 
        def myBuildTree(preorder_left, preorder_right, inorder_left, inorder_right):
            if preorder_left > preorder_right:
                return None
            
            # 前序遍历中的第一个节点就是根节点
            preorder_root = preorder_left
            # 在中序遍历中定位根节点
            inorder_root = index[preorder[preorder_root]]
            
            # 先把根节点建立出来
            root = TreeNode(preorder[preorder_root])
            
            # 得到左子树中的节点数目
            size_left_subtree = inorder_root - inorder_left
            
            # 递归地构造左子树，并连接到根节点
            # 先序遍历中「从 左边界+1 开始的 size_left_subtree」个元素就对应了中序遍历中「从 左边界 开始到 根节点定位-1」的元素
            root.left = myBuildTree(preorder_left + 1, 
                                    preorder_left + size_left_subtree, 
                                    inorder_left, 
                                    inorder_root - 1)
            
            # 递归地构造右子树，并连接到根节点
            # 先序遍历中「从 左边界+1+左子树节点数目 开始到 右边界」的元素就对应了中序遍历中「从 根节点定位+1 到 右边界」的元素
            root.right = myBuildTree(preorder_left + size_left_subtree + 1, 
                                    preorder_right, 
                                    inorder_root + 1, 
                                    inorder_right)
            return root


        n = len(preorder)
        index = {element: i for i, element in enumerate(inorder)}
        return myBuildTree(0, n - 1, 0, n - 1)