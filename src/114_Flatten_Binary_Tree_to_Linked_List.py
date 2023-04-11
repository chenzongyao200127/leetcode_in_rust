# 114_Flatten_Binary_Tree_to_Linked_List
# https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
# If you notice carefully in the flattened tree, each node's right child points to the next node of a pre-order traversal.
class Solution:
    def flatten(self, root: TreeNode) -> None:
        preorderList = list()

        def preorderTraversal(root: TreeNode):
            if root:
                preorderList.append(root)
                preorderTraversal(root.left)
                preorderTraversal(root.right)
        
        preorderTraversal(root)
        
        size = len(preorderList)
        
        for i in range(1, size):
            prev, curr = preorderList[i - 1], preorderList[i]
            prev.left = None
            prev.right = curr


# 以下是 flatten 方法的详细步骤：

# 如果根节点为空，直接返回。
# 递归地展平左子树。
# 存储原右子树的引用。
# 将左子树移动到右子树的位置，并将左子树置空。
# 遍历新的右子树，直到找到末尾节点。
# 递归地展平原右子树。
# 将原右子树接到新右子树的末尾。
# 此实现大致上是一种深度优先搜索策略，它先处理左子树，然后处理右子树。
class Solution:
    def flatten(self, root: TreeNode) -> None:
        if root is None:
            return
        
        self.flatten(root.left)
        
        right = root.right
        root.right = root.left
        root.left = None
        
        while root.right is not None:
            root = root.right
        
        self.flatten(right)
        root.right = right
        
# class Solution {
#     public void flatten(TreeNode root) {
#         if (root == null) return;
#         flatten(root.left);
#         TreeNode right = root.right;
#         root.right = root.left;
#         root.left = null;
#         while (root.right != null) {
#             root = root.right;
#         }
#         flatten(right);
#         root.right = right;
#     }
# }


class Solution:
    def flatten(self, root: TreeNode) -> None:
        if not root:
            return
        
        stack = [root]
        prev = None
        
        while stack:
            curr = stack.pop()
            
            if prev:
                prev.left = None
                prev.right = curr
                
            left, right = curr.left, curr.right
            
            if right:
                stack.append(right)
            if left:
                stack.append(left)
                
            prev = curr


# 它使用 Morris 遍历（一种线索二叉树遍历方法）的思想，遍历整个树并修改指针。
# 以下是 flatten 方法的详细步骤：

# 1. 初始化一个 curr 指针，指向根节点。
# 2. 当 curr 不为空时，进行以下操作：
#   1. 如果 curr 的左子节点不为空：
#       1. 初始化一个 predecessor 和 nxt 指针，指向 curr 的左子节点。
#       2. 寻找 curr 的左子树中的最右侧节点，即寻找 curr.left 的前驱节点。
#       3. 将前驱节点的右子节点设置为 curr 的右子节点。
#       4. 将 curr 的左子节点设置为 None。
#       5. 将 curr 的右子节点设置为 nxt。
#   2. 更新 curr 指针，使其指向当前节点的右子节点。
# 3. 遍历结束后，树将展平为链表。
class Solution:
    def flatten(self, root: TreeNode) -> None:
        curr = root
        
        while curr:
            if curr.left:
                predecessor = nxt = curr.left
                
                while predecessor.right:
                    predecessor = predecessor.right
                    
                predecessor.right = curr.right
                
                curr.left = None
                curr.right = nxt
                
            curr = curr.right