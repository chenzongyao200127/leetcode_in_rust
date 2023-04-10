# 106_Construct_Binary_Tree_from_Inorder_and_Postorder_Traversal
# https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
        if not inorder or not postorder:
            return None
        
        root = TreeNode(postorder[0])
        idx = inorder.index(root.val)
        
        left_inorder = inorder[:idx]
        right_inorder = inorder[idx+1:]
        
        left_postorder = postorder[:len(left_inorder)]
        right_postorder = postorder[len(left_postorder):-1]
        
        root.left = self.buildTree(left_inorder, left_postorder);
        root.right = self.buildTree(right_inorder, right_postorder);
        
        return root
        
        
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
        # 仍然是通过找到中序中的根节点然后将树一分为二，但是后序和前序的区别在于根节点从尾部开始
        N=len(inorder)
        inhash=dict() # 哈希表存储中序列表中每个值的下标，不用每次重新遍历
        for i in range(N):
            inhash[inorder[i]]=i
        
        def helper(inStart,inEnd,poStart,poEnd):
            if inStart>inEnd:
                return None
            root=TreeNode(postorder[poEnd])
            inIndex=inhash[postorder[poEnd]]
            rightTreesize=inEnd-inIndex # 右子树大小
            root.left=helper(inStart,inIndex-1,poStart,poEnd-rightTreesize-1)
            root.right=helper(inIndex+1,inEnd,poEnd-rightTreesize,poEnd-1)
            return root
        
        return helper(0,N-1,0,N-1)

